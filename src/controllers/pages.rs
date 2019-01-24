use super::prelude::*;
use crate::views::pages::*;

pub fn index(req: &HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .body(index_view(req.uri().path()))
}
