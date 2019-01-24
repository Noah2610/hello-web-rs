mod prelude {
    pub use actix_web::Responder;
    pub use maud::{ Markup, PreEscaped };
    pub use super::layouts::*;
}

pub mod layouts;
pub mod not_found;
pub mod pages;
pub mod posts;
