mod config;
mod headers;
mod post;
mod routes;
mod render;

use tiny_http::Server;
use config::PORT;
use routes::handle_request;


fn main() {
  let server = Server::http(format!("0.0.0.0:{}", PORT)).unwrap();
  println!("Server is running on http://localhost:{}", PORT);

  for request in server.incoming_requests() {
    println!("{}", request.url());
    handle_request(request);
  }
}
