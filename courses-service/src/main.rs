use std::env;

use chrono::Utc;
use sqlx::postgres::{PgPoolOptions, Postgres};
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

use schema::courses::{courses_server::{Courses, CoursesServer}};

#[derive(Debug)]
pub struct CoursesService<T>
where
    for<'a> &'a T: sqlx::Executor<'a, Database = Postgres>,
{
    executor: T,
}

impl<T> CoursesService<T>
where
    for<'a> &'a T: sqlx::Executor<'a, Database = Postgres>,
{
    pub fn new(executor: T) -> Self {
        Self { executor }
    }
}

#[tonic::async_trait]
impl<T: Send + Sync + 'static> Courses for CoursesService<T>
where
    for<'a> &'a T: sqlx::Executor<'a, Database = Postgres>,
{
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

    let service = CoursesService::new(pool);

    Server::builder()
        .add_service(CoursesServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
