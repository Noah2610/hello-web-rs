#![feature(proc_macro_hygiene)]

extern crate actix_web;
extern crate colored;
#[macro_use]
extern crate maud;

pub mod controllers;
pub mod views;

use actix_web::{
    server,
    fs,
    App,
    HttpResponse,
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
                 .resource("/",          |r| r.f(cnt::pages::index) )
                 .handler("/resources", fs::StaticFiles::new("./resources").unwrap())
                 .default_resource(      |r| r.f(cnt::not_found::get) ))
        .bind(&to_bind)
        .expect("Should bind to address")
        .run();
}
