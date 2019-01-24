// private, because this should only ever be used by controllers;
// controllers being modules laying below this module.
mod prelude {
    pub use actix_web::{
        HttpRequest,
        HttpResponse,
        Responder,
        http::ContentEncoding,
    };
}

pub mod not_found;
pub mod pages;
