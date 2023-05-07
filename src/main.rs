use std::sync::Mutex;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use tokio::{fs::File, io::{BufReader, AsyncBufReadExt}};
use serde::Deserialize;
use actix_web::{get, main as actix_main, App, HttpServer, Result, middleware::Logger, web::Data};
use actix_files::NamedFile;
use env_logger::Env;

mod utils;

fn def_port() -> u16 { 8080 }

fn def_deta_space_app() -> bool { false }

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "def_deta_space_app")]
    deta_space_app: bool,
    #[serde(default = "def_port")]
    port: u16,
}

#[get("/")]
async fn random(rng: Data<Mutex<ThreadRng>>) -> Result<String> {
    let lines = BufReader::new(File::open("ua.txt").await?).lines();
    let r: f64 = rng.lock().unwrap().gen();
    Ok(utils::one_pass_skip(lines, r).await?.unwrap())
}

#[get("/all")]
async fn all() -> Result<NamedFile> { Ok(NamedFile::open("ua.txt")?) }

#[actix_main]
async fn main() -> std::io::Result<()> {
    let config = envy::from_env::<Config>().unwrap();
    if !config.deta_space_app {
        env_logger::init_from_env(Env::default().default_filter_or("info"));
    }

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(Mutex::new(thread_rng()))) // thread local generator
            .wrap(Logger::default())
            .service(random)
            .service(all)
    })
        .bind(("127.0.0.1", config.port))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_web::{test, body::to_bytes};

    use super::*;

    #[test]
    async fn test_get_random() {
        let app = test::init_service(App::new()
            .app_data(Data::new(Mutex::new(thread_rng())))
            .service(random)).await;
        let req = test::TestRequest::default().to_request();
        let res = test::call_service(&app, req).await;
        let req = test::TestRequest::default().to_request();
        let res1 = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert!(res1.status().is_success());

        let body = to_bytes(res.into_body()).await.unwrap();
        let body1 = to_bytes(res1.into_body()).await.unwrap();

        assert_ne!(body, body1);
    }

    #[test]
    async fn test_get_all() {
        let app = test::init_service(App::new().service(all)).await;
        let req = test::TestRequest::get().uri("/all").to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.headers().get("content-disposition").unwrap(), "inline; filename=\"ua.txt\"");
        assert_eq!(res.headers().get("content-type").unwrap(), "text/plain; charset=utf-8");
    }
}
