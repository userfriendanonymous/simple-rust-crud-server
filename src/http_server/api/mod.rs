mod posts;
use actix_web::{web, Scope};

pub fn main() -> Scope {
    web::scope("posts")
    .service(posts::create)
    .service(posts::get_all)
    .service(posts::get_one)
}