use maud::{ DOCTYPE, Markup };

use crate::PAGE_TITLE;

fn compose_title(sub: &str) -> String {
    format!("{} | {}", sub, PAGE_TITLE)
}

pub fn header(title: &str) -> Markup {
    html! {
        (DOCTYPE);
        head {
            meta charset="utf-8";
            title { (compose_title(title)) }
            link rel="stylesheet" type="text/css" href="/resources/lib/css/bootstrap.min.css";
            script type="text/javascript" src="/resources/lib/js/bootstrap.min.js" {}
        }
    }
}

pub fn navigation(current_uri: Option<&str>) -> Markup {
    html! {
        div.container {
            ul.nav.nav-pills {
                li.active[is_uri(current_uri, "/")] {
                    a href="/" { "Home" }
                }
                li.active[is_uri(current_uri, "/one")] {
                    a href="/posts" { "Posts" }
                }
                li.active[is_uri(current_uri, "/two")] {
                    a href="/posts/new" { "New Post" }
                }
            }
        }
    }
}

fn is_uri(current_uri: Option<&str>, to_check: &str) -> bool {
    if let Some(uri) = current_uri {
        uri == to_check
    } else {
        false
    }
}
