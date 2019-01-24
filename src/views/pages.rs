use super::prelude::*;

pub fn index_view(uri: &str) -> String {
    (html! {
        (header("Index"))
        (navigation(Some(uri)))
        div.container {
            img src="/resources/img.png";
            div.well {
                p {
                    "Hello World!"
                }
            }
        }
    }).into_string()
}
