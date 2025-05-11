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
    Ok(())

}   
