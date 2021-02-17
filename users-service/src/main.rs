use std::env;

use bcrypt::{hash, verify};
use chrono::Utc;
use sqlx::postgres::{PgPoolOptions, Postgres};
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

use schema::{
    shared::{User, UserRole},
    users::{
        users_server::{Users, UsersServer},
        AuthenticateRequest, AuthenticateResponse, CreateUserRequest, CreateUserResponse,
        GetAllUsersRequest, GetTokenRequest, GetTokenResponse, GetUsersByIdsRequest,
        GetUsersByIdsResponse, UpdateUserRoleRequest, UpdateUserRoleResponse,
    },
};

mod errors;
mod jwt;

use errors::UsersServiceError;

#[derive(Debug)]
pub struct UsersService<T>
where
    for<'a> &'a T: sqlx::Executor<'a, Database = Postgres>,
{
    executor: T,
}

impl<T> UsersService<T>
where
    for<'a> &'a T: sqlx::Executor<'a, Database = Postgres>,
{
    pub fn new(executor: T) -> Self {
        Self { executor }
    }
}

#[tonic::async_trait]
impl<T: Send + Sync + 'static> Users for UsersService<T>
where
    for<'a> &'a T: sqlx::Executor<'a, Database = Postgres>,
{
    type GetAllUsersStream = mpsc::Receiver<Result<User, Status>>;

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let req = request.into_inner();
        let user = (sqlx::query!(
            "INSERT INTO users (
                username,
                password,
                created_at,
                updated_at,
                user_role
            ) VALUES ($1, $2, $3, $4, $5) RETURNING *;",
            req.username,
            hash(&req.password, 10).unwrap(),
            Utc::now(),
            Utc::now(),
            UserRole::Standard as i32
        )
        .fetch_one(&self.executor)
        .await)
            .map_err(UsersServiceError::from)?;

        Ok(Response::new(CreateUserResponse {
            user: Some(User {
                id: user.id,
                username: user.username.to_owned(),
                role: user.user_role,
            }),
        }))
    }

    async fn get_token(
        &self,
        request: Request<GetTokenRequest>,
    ) -> Result<Response<GetTokenResponse>, Status> {
        let req = request.into_inner();
        let user = sqlx::query!("SELECT * FROM users WHERE username=$1", req.username)
            .fetch_one(&self.executor)
            .await
            .map_err(UsersServiceError::from)?;
        if verify(req.password, user.password.as_str()).map_err(UsersServiceError::from)? {
            Ok(Response::new(GetTokenResponse {
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
        .fetch_all(&self.executor)
        .await
        .map_err(UsersServiceError::from)?;

        Ok(Response::new(GetUsersByIdsResponse {
            users: users
                .into_iter()
                .map(|user| User {
                    id: user.id,
                    username: user.username.to_owned(),
                    role: user.user_role,
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
            .fetch_all(&self.executor)
            .await
            .map_err(UsersServiceError::from)?;

        tokio::spawn(async move {
            for user in users {
                tx.send(Ok(User {
                    id: user.id,
                    username: user.username,
                    role: user.user_role,
                }))
                .await
                .unwrap();
            }
        });

        Ok(Response::new(rx))
    }

    async fn authenticate(
        &self,
        request: Request<AuthenticateRequest>,
    ) -> Result<Response<AuthenticateResponse>, Status> {
        let result = jwt::verify_jwt(request.into_inner().token).unwrap();

        let user = sqlx::query!("SELECT * FROM users WHERE id=$1;", result.claims.user_id)
            .fetch_one(&self.executor)
            .await
            .map_err(UsersServiceError::from)?;

        log::info!(
            "Verified request from user {} with id {}",
            user.username,
            user.id
        );

        Ok(Response::new(AuthenticateResponse {
            user: Some(User {
                username: user.username,
                id: user.id,
                role: user.user_role,
            }),
        }))
    }

    async fn update_user_role(
        &self,
        request: Request<UpdateUserRoleRequest>,
    ) -> Result<Response<UpdateUserRoleResponse>, Status> {
        let message = request.into_inner();
        if message.active_user.map(|u| u.role).unwrap_or(0) == UserRole::Administrator as i32 {
            sqlx::query!(
                "UPDATE users SET user_role=$1 WHERE id=$2;",
                message.new_role,
                message.user_id
            )
            .execute(&self.executor)
            .await
            .map_err(UsersServiceError::from)?;

            Ok(Response::new(UpdateUserRoleResponse { success: true }))
        } else {
            Ok(Response::new(UpdateUserRoleResponse { success: false }))
        }
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
