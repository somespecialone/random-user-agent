use actix_cors::Cors;

pub fn cors() -> Cors {
    Cors::default()
        .allow_any_header()
        .allowed_methods(vec!["GET"])
        .allow_any_origin()
        .send_wildcard()
}
