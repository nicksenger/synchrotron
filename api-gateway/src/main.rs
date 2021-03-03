use std::{env, io, path::PathBuf, sync::Arc};

use actix_files::NamedFile;
use actix_web::{
    middleware::Logger,
    web::{get, post},
    App, HttpRequest, HttpServer,
};
use dotenv::dotenv;
use structopt::StructOpt;

use gateway::{graphql, AppData};

#[derive(Debug, StructOpt)]
#[structopt(name = "microbiome_server")]
struct Opt {
    #[structopt(short = "s", long = "socket", default_value = "0.0.0.0:8000")]
    socket: String,
}

async fn index(req: HttpRequest) -> io::Result<NamedFile> {
    let mut path = req
        .match_info()
        .query("path")
        .split("/")
        .collect::<Vec<&str>>();
    let seg = path.pop().unwrap_or("");
    if seg.find('.').is_some() {
        Ok(NamedFile::open::<PathBuf>(
            format!("./static/{}/{}", path.join("/"), seg).parse().unwrap(),
        )?)
    } else {
        Ok(NamedFile::open::<PathBuf>(
            "./static/index.html".parse().unwrap(),
        )?)
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let opt = Opt::from_args();
    let url = opt.socket;

    let user_channel =
        tonic::transport::Channel::from_shared(env::var("USERS_SERVICE_URI").unwrap())
            .unwrap()
            .connect()
            .await
            .unwrap();
    let courses_channel =
        tonic::transport::Channel::from_shared(env::var("COURSES_SERVICE_URI").unwrap())
            .unwrap()
            .connect()
            .await
            .unwrap();

    let schema = Arc::new(graphql::schema::create_schema());

    log::info!("Microbiome running at: http://{}", url);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(AppData {
                schema: schema.clone(),
                user_channel: user_channel.clone(),
                courses_channel: courses_channel.clone(),
            })
            .route("/graphql", post().to(graphql::handler::graphql))
            .route("/graphiql", get().to(graphql::handler::graphiql))
            .route("{path:.*}", get().to(index))
    })
    .bind(&url)
    .expect("Failed to start Microbiome")
    .run()
    .await
}
