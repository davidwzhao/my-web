extern crate actix_web;
use actix_web::{server, App, http, HttpRequest, HttpResponse};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn readFromFile(fileName: &'static str) -> std::io::Result<String> {
    // read from file
    let file = File::open(fileName)?;
    let mut bufReader = BufReader::new(file);

    let mut fileContents = String::new();
    bufReader.read_to_string(&mut fileContents)?;

    Ok(fileContents)
}



fn index(_req: &HttpRequest) -> HttpResponse {
    // let s = "<!DOCTYPE html>\n<html>\n<head><title>LOLOL</title></head><body>\n<b>Hello</b>\n</body>\n</html>";
    let s = match readFromFile("test.html") {
        Ok(s) => s,
        e => String::from("Error :("),
    };
    
    return HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "text/html")
        .body(s);
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}
