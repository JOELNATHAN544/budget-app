mod db;
mod handlers;
mod models;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use db::AppState;
use handlers::{add_transaction, delete_transaction, get_transactions};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let pool = db::init_db().await.expect("Failed to connect to database");
    let app_state = web::Data::new(AppState { pool });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allowed_headers(vec!["Content-Type"]);
            // .allow_headers(vec!["Content-Type"]);

        App::new().wrap(cors).app_data(app_state.clone()).service(
            web::scope("/api")
                .route("/transactions", web::get().to(get_transactions))
                .route("/transactions", web::post().to(add_transaction))
                .route("/transactions/{id}", web::delete().to(delete_transaction)),
        )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
