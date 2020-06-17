use actix_cors::Cors;
use actix_web::http;

pub fn cors_options() -> actix_cors::CorsFactory {
    Cors::new() // <- Construct CORS middleware builder
        .allowed_origin("http://localhost:3000")
        .allowed_origin("https://localhost:5500")
        .allowed_origin("http://localhost:5500")
        .allowed_origin("https://api.baizuo.online")
        .allowed_origin("http://api.baizuo.online")
        .allowed_origin("http://kantimam.org")
        .allowed_origin("https://kantimam.org")
        .allowed_origin("https://romantic-sinoussi-84727e.netlify.app/")
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
        .finish()
}
