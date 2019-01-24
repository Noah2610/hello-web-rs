use diesel::prelude::*;

use super::prelude::*;
use crate::db;
use crate::schema::posts::dsl::*;
use crate::views::posts::*;
use crate::models::Post;

#[derive(Deserialize)]
struct FormData {

}

pub fn index(req: &HttpRequest) -> impl Responder {
    let connection = db::establish_connection();
    let filtered_posts = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    HttpResponse::Ok()
        .body(
            index_view(req.uri().path(), filtered_posts)
        )
}

pub fn new(req: &HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .body(new_view(req.uri().path()))
}

pub fn create(req: &HttpRequest) -> impl Responder {
    println!("{:?}", req.query());
    "foo"
}
