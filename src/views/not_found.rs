use super::prelude::*;

pub fn get_view() -> String {
    (html! {
        (header("404 Not Found"));
        h1 {
            "404"
        }
        h2 {
            "Page Not Found"
        }
    }).into_string()
}
