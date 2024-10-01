use maud::{html, Markup, PreEscaped};

use crate::config::{BLOG_NAME, DESCRIPTION, POSTS_PER_PAGE};
use crate::post::{POSTS, Post, TimeType};


pub enum PageType<'a> {
  HomePage {
    posts: &'a [Post],
    current_page: usize,
  },
  PostPage {
    post: &'a Post,
  },
}

pub fn render_page(page: PageType) -> Markup {
  html! {
    head {
      title {
        @match &page {
          PageType::HomePage { .. } => (BLOG_NAME.as_str()),
          PageType::PostPage { post } => (format!("{} - {}", post.title, BLOG_NAME.as_str())),
        }
      }
    }
    body {
      header {
        h1 { a href={"/"} { (BLOG_NAME.as_str()) } }
        @match &page {
          PageType::HomePage { .. } => {
            p { (DESCRIPTION.as_str()) }
          },
          _ =>  {},
        }
        nav {}
      }
      @match &page {
        PageType::HomePage { posts, current_page } => {
          section {
            @for post in posts.iter().filter(|p| p.published) {
              article {
                h2 { a href=(format!("/{}", post.path)) { (PreEscaped(&post.title)) } }
                time { (post.format_time(TimeType::Created)) }
                div { (PreEscaped(&post.preview)) }
                a href=(format!("/{}", post.path)) { "Read more" }
              }
            }
            @let total_pages = (POSTS.read().unwrap().len() as f32 / POSTS_PER_PAGE as f32).ceil() as usize;
            nav {
              @if *current_page < total_pages {
                a href={(format!("/page/{}", current_page + 1))} { "Older" }
              }
              " | "
              @if *current_page > 1 {
                a href={(format!("/page/{}", current_page - 1))} { "Newer" }
              }
            }
          }
        },
        PageType::PostPage { post } => {
          article {
            h2 { (PreEscaped(&post.title)) }
            time { (post.format_time(TimeType::Created)) }
            @if post.time_diff_minutes() > 5.0 {
              " | "
              time { "Updated: " (post.format_time(TimeType::Modified)) }
            }
            div { (PreEscaped(&post.content)) }
          }
        },
      }
      footer {
        hr {}
        nav {
          p { "© 2024 fashni" }
        }
      }
    }
  }
}
