mod config;
mod db;
mod handlers;
mod models;
mod multi_part_handler;
use actix_cors::Cors;
use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;
use handlebars::Handlebars;
use std::io;
use tokio_postgres::NoTls;

use crate::handlers::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().expect("could not get env vars :(");

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    // create or update admin account
    create_admin(&pool).await;

    // setupt templating engine
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", config.directory.templates)
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .app_data(handlebars_ref.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false)
                    .max_age(86400), // 1 day in seconds
            ))
            .wrap(
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
                    .finish(),
            )
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/projects")
                            .route(web::get().to(get_projects))
                            .route(web::post().to(create_project)),
                    )
                    .service(
                        web::resource("/projectslist").route(web::get().to(get_projects_template)),
                    )
                    .service(
                        web::resource("/projectform").route(web::get().to(create_project_template)),
                    )
                    .service(
                        web::resource("/login")
                            .route(web::post().to(log_in))
                            .route(web::get().to(log_in_template)),
                    )
                    .service(web::resource("/logout").route(web::get().to(log_out)))
                    .service(web::resource("/status").route(web::get().to(status)))
                    .service(web::resource("/sendmail").route(web::post().to(send_mail)))
                    .service(fs::Files::new(
                        "/static",
                        std::env::var("DIRECTORY.STATIC_FILES")
                            .expect("DIRECTORY.STATIC_FILES must be set in the .env variables"),
                    ))
                    .default_service(web::route().to(index_template)),
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
