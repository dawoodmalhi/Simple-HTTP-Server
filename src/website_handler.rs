use super::http::{Response, Request, StatusCode};
use super::server::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
  fn handle_request(&mut self, request: &Request) -> Response {
    Response::new(StatusCode::Ok, Some("PropheC. Success".to_string()))
  }
}