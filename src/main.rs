mod config;
mod headers;
mod post;
mod routes;
mod render;

use std::{io, process, thread};
use tiny_http::Server;
use config::CONFIG;
use post::reload_posts;
use routes::handle_request;


fn main() {
  let cl_thread = thread::spawn(|| command_listener());
  start_server();
  cl_thread.join().unwrap();
}

fn start_server() {
  let server = Server::http(format!("0.0.0.0:{}", CONFIG.port)).unwrap();
  println!("Server is running on http://localhost:{}", CONFIG.port);

  for request in server.incoming_requests() {
    println!("{}", request.url());
    handle_request(request);
  }
}

fn command_listener() {
  loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim() {
      "r" => {
        reload_posts();
        println!("Posts reloaded");
      },
      "q" => process::exit(0),
      _ => println!("Invalid command"),
    }
  }
}
