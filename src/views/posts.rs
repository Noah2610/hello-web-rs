use super::prelude::*;
use crate::models::Post;

pub fn index_view(uri: &str, posts: Vec<Post>) -> String {
    (html! {
        (header("Posts"));
        (navigation(Some(uri)));
        h1 { "Published Posts" }
        (PreEscaped(posts.iter()
                    .map( post_entry ).collect::<String>()))
    }).into_string()
}

fn post_entry(post: &Post) -> String {
    (html! {
        div.panel.panel-default {
            div.panel-heading { (post.title) }
            div.panel-body    { (post.body)  }
            div.panel-footer  { "Published: " strong { (post.published) } }
        }
    }).into_string()
}

pub fn new_view(uri: &str) -> String {
    (html! {
        (header("New Post"));
        (navigation(Some(uri)));
        div.container {
            h1 { "Create a new Post!" }
            form action="/posts/create" method="post" {
                label.control-label for="title" { "Post Title" }
                input.form-control type="text" name="title" placeholder="Post Title" {}
                label.control-label for="body" { "Post Content" }
                textarea.form-control name="body" placeholder="Post Content" {}
                input.btn.btn-primary type="submit" value="Create Post!" { }
            }
        }
    }).into_string()
}
