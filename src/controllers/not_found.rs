use super::prelude::*;
use crate::views::not_found::*;

pub fn get(_req: &HttpRequest) -> impl Responder {
    HttpResponse::NotFound()
        .body(get_view())
}
