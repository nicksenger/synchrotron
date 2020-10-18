use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

use schema::users::{
    users_server::{Users, UsersServer},
    AuthenticateRequest, AuthenticateResponse, CreateUserRequest, CreateUserResponse,
    GetAllUsersRequest, GetUsersByIdsRequest, GetUsersByIdsResponse, User,
};

#[derive(Debug, Default)]
pub struct UsersService;

#[tonic::async_trait]
impl Users for UsersService {
    type GetAllUsersStream = mpsc::Receiver<Result<User, Status>>;

    async fn create_user(
        &self,
        _request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        Ok(Response::new(CreateUserResponse {
            user: Some(User {
                id: 1,
                username: "foo".to_owned(),
            }),
        }))
    }

    async fn authenticate(
        &self,
        _request: Request<AuthenticateRequest>,
    ) -> Result<Response<AuthenticateResponse>, Status> {
        Ok(Response::new(AuthenticateResponse {
            token: "abc123".to_owned(),
        }))
    }

    async fn get_users_by_ids(
        &self,
        request: Request<GetUsersByIdsRequest>,
    ) -> Result<Response<GetUsersByIdsResponse>, Status> {
        Ok(Response::new(GetUsersByIdsResponse {
            users: request
                .into_inner()
                .user_ids
                .into_iter()
                .map(|id| User {
                    id,
                    username: "foo".to_owned(),
                })
                .collect(),
        }))
    }

    async fn get_all_users(
        &self,
        _request: Request<GetAllUsersRequest>,
    ) -> Result<Response<Self::GetAllUsersStream>, Status> {
        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            for user in Vec::<User>::new() {
                tx.send(Ok(user.clone())).await.unwrap();
            }
        });

        Ok(Response::new(rx))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = UsersService::default();

    Server::builder()
        .add_service(UsersServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
