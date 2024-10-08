use std::{fs, path::Path};
use tiny_http::{Header, Request, Response, StatusCode};

use crate::config::CONFIG;
use crate::post::POSTS;
use crate::headers;
use crate::render::{PageType, render_page};


pub fn handle_request(request: Request) {
  let url = request.url().to_string();

  if url == "/" || url.starts_with("/page/") {
    let page = if url == "/" {1} else {
      url.trim_start_matches("/page/").parse::<usize>().unwrap_or(1)
    };

    let all_posts = POSTS.read().unwrap();
    let start = (page - 1) * CONFIG.posts_per_page;
    let end = start + CONFIG.posts_per_page;
    let posts = &all_posts[start..all_posts.len().min(end)];

    let homepage = render_page(PageType::HomePage {
      posts: posts,
      current_page: page
    });

    let response = Response::from_string(homepage.into_string())
      .with_header(headers::HTML_HEADER.clone());
    request.respond(response).unwrap();
  } else if url.starts_with("/static/") {
    serve_static(&url[1..], request);
  } else {
    let path = &url[1..];
    if let Some(post) = POSTS.read().unwrap().iter().find(|p| p.path == path && p.published) {
      let postpage = render_page(PageType::PostPage { post: post });
      let response = Response::from_string(postpage.into_string())
        .with_header(headers::HTML_HEADER.clone());
      request.respond(response).unwrap();
    } else {
      let response = Response::from_string("404 Not Found")
        .with_status_code(StatusCode(404))
        .with_header(headers::HTML_HEADER.clone());
      request.respond(response).unwrap();
    }
  }
}

fn serve_static(file_path: &str, request: Request) {
  if let Ok(content) = fs::read(file_path) {
    let header = get_header(file_path);
    let response = Response::from_data(content)
      .with_header(header.clone());
    request.respond(response).unwrap();
  } else {
    let response = Response::from_string("404 Not Found")
      .with_status_code(StatusCode(404))
      .with_header(headers::HTML_HEADER.clone());
    request.respond(response).unwrap();
  }
}

fn get_header(file_path: &str) -> &'static Header {
  match Path::new(file_path).extension().and_then(|ext| ext.to_str()) {
      Some("html") => &headers::HTML_HEADER,
      Some("css") => &headers::CSS_HEADER,
      Some("js") => &headers::JS_HEADER,
      Some("png") => &headers::PNG_HEADER,
      Some("jpg") | Some("jpeg") => &headers::JPEG_HEADER,
      Some("gif") => &headers::GIF_HEADER,
      Some("svg") => &headers::SVG_HEADER,
      Some("ico") => &headers::ICO_HEADER,
      Some("woff") => &headers::WOFF_HEADER,
      Some("woff2") => &headers::WOFF2_HEADER,
      _ => &headers::OCTET_STREAM_HEADER,
  }
}
