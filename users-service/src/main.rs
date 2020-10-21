use std::env;

use bcrypt::{hash, verify};
use chrono::Utc;
use sqlx::{
    postgres::{PgPoolOptions, Postgres},
    Pool,
};
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

use schema::users::{
    users_server::{Users, UsersServer},
    AuthenticateRequest, AuthenticateResponse, CreateUserRequest, CreateUserResponse,
    GetAllUsersRequest, GetUsersByIdsRequest, GetUsersByIdsResponse, User, VerifyRequest,
    VerifyResponse,
};

mod errors;
mod jwt;

use errors::UsersServiceError;

#[derive(Debug)]
pub struct UsersService {
    pool: Pool<Postgres>,
}

impl UsersService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl Users for UsersService {
    type GetAllUsersStream = mpsc::Receiver<Result<User, Status>>;

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();
        (sqlx::query!(
            "INSERT INTO users (
                username,
                password,
                created_at,
                updated_at
            ) VALUES ($1, $2, $3, $4);",
            req.username,
            hash(&req.password, 10).unwrap(),
            Utc::now(),
            Utc::now(),
        )
        .execute(&self.pool)
        .await)
            .map_err(UsersServiceError::from)?;

        let user = sqlx::query!("SELECT * FROM users WHERE username=$1;", req.username)
            .fetch_one(&self.pool)
            .await
            .map_err(UsersServiceError::from)?;

        Ok(Response::new(CreateUserResponse {
            user: Some(User {
                id: user.id,
                username: user.username.to_owned(),
            }),
        }))
    }

    async fn authenticate(
        &self,
        request: Request<AuthenticateRequest>,
    ) -> Result<Response<AuthenticateResponse>, Status> {
        let req = request.into_inner();
        let user = sqlx::query!("SELECT * FROM users WHERE username=$1", req.username)
            .fetch_one(&self.pool)
            .await
            .map_err(UsersServiceError::from)?;
        if verify(req.password, user.password.as_str()).map_err(UsersServiceError::from)? {
            Ok(Response::new(AuthenticateResponse {
                token: jwt::encode_jwt(user.id, 30).unwrap(),
            }))
        } else {
            Err(Status::permission_denied("Invalid login"))
        }
    }

    async fn get_users_by_ids(
        &self,
        request: Request<GetUsersByIdsRequest>,
    ) -> Result<Response<GetUsersByIdsResponse>, Status> {
        let req = request.into_inner();
        let users = sqlx::query!(
            "SELECT * FROM users WHERE id IN (SELECT * FROM UNNEST($1::int[]));",
            &req.user_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(UsersServiceError::from)?;

        Ok(Response::new(GetUsersByIdsResponse {
            users: users
                .into_iter()
                .map(|user| User {
                    id: user.id,
                    username: user.username.to_owned(),
                })
                .collect(),
        }))
    }

    async fn get_all_users(
        &self,
        _request: Request<GetAllUsersRequest>,
    ) -> Result<Response<Self::GetAllUsersStream>, Status> {
        let (mut tx, rx) = mpsc::channel(4);

        let users = sqlx::query!("SELECT * FROM users;")
            .fetch_all(&self.pool)
            .await
            .map_err(UsersServiceError::from)?;

        tokio::spawn(async move {
            for user in users {
                tx.send(Ok(User {
                    id: user.id,
                    username: user.username,
                }))
                .await
                .unwrap();
            }
        });

        Ok(Response::new(rx))
    }

    async fn verify(
        &self,
        request: Request<VerifyRequest>,
    ) -> Result<Response<VerifyResponse>, Status> {
        let result = jwt::verify_jwt(request.into_inner().token).unwrap();

        let user = sqlx::query!("SELECT * FROM users WHERE id=$1;", result.claims.user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(UsersServiceError::from)?;

        log::info!(
            "Verified request from user {} with id {}",
            user.username,
            user.id
        );

        Ok(Response::new(VerifyResponse { user_id: user.id }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let addr = "[::0]:50051".parse()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    let service = UsersService::new(pool);

    Server::builder()
        .add_service(UsersServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
