use actix_cors::Cors;
use actix_web::{HttpServer, middleware::Logger, App, http::header};
use sqlx::{Postgres, Pool, postgres::PgPoolOptions};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        } 
    }
    dotenv().ok();
    env_logger::init();

    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool:Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("connection to the db is successful!");
            pool
        }
        Err(err) => {
            println!("failed to connect to the database {:?}", err);
            std::process::exit(1);
        }
    };

    println!("server started successfully!");

    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_origin("http://localhost:3000")
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::CONTENT_ENCODING,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    Ok(())

}   
