use std::sync::Mutex;

use actix_files::NamedFile;
use actix_web::{
    get,
    http::header::{CacheControl, CacheDirective, ContentType},
    web::Data,
    HttpResponse, Result,
};
use rand::{rngs::ThreadRng, Rng};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

use crate::utils;

#[get("/")]
async fn random(rng: Data<Mutex<ThreadRng>>) -> Result<HttpResponse> {
    let lines = BufReader::new(File::open("ua.txt").await?).lines();
    let r: f64 = rng.lock().unwrap().gen();
    let result = utils::one_pass_skip(lines, r).await?.unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(CacheControl(vec![
            CacheDirective::NoStore,
            CacheDirective::NoCache,
            CacheDirective::MaxAge(0),
            CacheDirective::MustRevalidate,
        ]))
        .body(result))
}

#[get("/all")]
async fn all() -> Result<NamedFile> {
    Ok(NamedFile::open("ua.txt")?)
}
