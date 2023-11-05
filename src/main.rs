use std::{io::Result, sync::Mutex};

use actix_web::{main as actix_main, middleware::Logger, web::Data, App, HttpServer};
use env_logger::{init_from_env, Env};
use rand::thread_rng;

mod config;
mod middleware;
mod routes;
mod utils;

#[actix_main]
async fn main() -> Result<()> {
    let config = envy::from_env::<config::Config>().unwrap();
    if !config.deta_space_app {
        init_from_env(Env::default().default_filter_or("info"));
    }

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(Mutex::new(thread_rng()))) // thread local generator
            .wrap(Logger::default())
            .wrap(middleware::cors())
            .service(routes::random)
            .service(routes::all)
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, test};

    use super::*;

    #[test]
    async fn test_get_random() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(Mutex::new(thread_rng())))
                .service(routes::random),
        )
        .await;
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
        let app = test::init_service(App::new().service(routes::all)).await;
        let req = test::TestRequest::get().uri("/all").to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(
            res.headers().get("content-disposition").unwrap(),
            "inline; filename=\"ua.txt\""
        );
        assert_eq!(
            res.headers().get("content-type").unwrap(),
            "text/plain; charset=utf-8"
        );
    }
}
