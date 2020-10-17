use std::{io, path::PathBuf, sync::Arc};

use actix_files::NamedFile;
use actix_web::{
    middleware::Logger,
    web::{get, post},
    App, HttpRequest, HttpServer,
};
use dotenv::dotenv;
use structopt::StructOpt;

mod data;
mod entities;
mod graphql;

#[derive(Debug, StructOpt)]
#[structopt(name = "glot_server")]
struct Opt {
    #[structopt(short = "s", long = "socket", default_value = "127.0.0.1:8000")]
    socket: String,
}

async fn index(req: HttpRequest) -> io::Result<NamedFile> {
    let mut path = req.match_info().query("path").split("/").peekable();
    match path.peek() {
        Some(&"") | None => Ok(NamedFile::open::<PathBuf>(
            "./static/index.html".parse().unwrap(),
        )?),
        Some(_) => Ok(NamedFile::open::<PathBuf>(
            format!("./static/{}", path.collect::<Vec<&str>>().join("/"))
                .parse()
                .unwrap(),
        )?),
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let opt = Opt::from_args();
    env_logger::init();
    let url = opt.socket;

    let schema = Arc::new(graphql::schema::create_schema());

    println!("Glot running at: http://{}", url);
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .route("/graphql", post().to(graphql::handler::graphql))
            .route("/graphiql", get().to(graphql::handler::graphiql))
            .wrap(Logger::default())
            .route("{path:.*}", get().to(index))
    })
    .bind(&url)
    .expect("Failed to start Glot")
    .run()
    .await
}
