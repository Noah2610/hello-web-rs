#![feature(proc_macro_hygiene)]

extern crate actix_web;
extern crate colored;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate maud;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod db;
pub mod schema;
pub mod models;
pub mod controllers;
pub mod views;

use actix_web::{
    server,
    fs,
    App,
};
use colored::*;

const ADDR:       &'static str = "127.0.0.1";
const PORT:       &'static str = "8080";
const PAGE_TITLE: &'static str = "Rust web dev!";

fn main() {
    start_server();
}

fn start_server() {
    use self::controllers as cnt;

    let to_bind = format!("{}:{}", ADDR, PORT);
    println!("Running on {}", to_bind.green().bold());
    server::new( || App::new()
                 .resource("/",              |r| r.f(cnt::pages::index)  )
                 .resource("/posts",         |r| r.f(cnt::posts::index)  )
                 .resource("/posts/new",     |r| r.f(cnt::posts::new)    )
                 .resource("/posts/create",  |r| r.f(cnt::posts::create) )
                 .handler("/resources", fs::StaticFiles::new("./resources").unwrap())
                 .default_resource(          |r| r.f(cnt::not_found::get) ))
        .bind(&to_bind)
        .expect("Should bind to address")
        .run();
}
