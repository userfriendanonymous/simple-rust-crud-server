mod api;
use sqlx::{Postgres, Pool};
use actix_web::{HttpServer, App, web};

use crate::core::{db, posts::Post};

struct AppState {
    db: Pool<Postgres>
}

pub async fn launch(db_url: &str) -> std::io::Result<()> {
    let db = db::create(db_url).await;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS posts (
            id SERIAL NOT NULL PRIMARY KEY,
            title VARCHAR(100),
            description VARCHAR(255),
            content TEXT
        )"
    )
    .execute(&db)
    .await
    .unwrap();

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState {
            db: db.clone()
        }))
        .service(
            web::scope("/api")
            .service(api::main())
        )
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}