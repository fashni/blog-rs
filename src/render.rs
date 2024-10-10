use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::config::CONFIG;
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
    (DOCTYPE)
    html lang="en" {
      head {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width,initial-scale=1.0";
        meta name="author" content=(CONFIG.author);
        title {
          @match &page {
            PageType::HomePage { .. } => (CONFIG.blog_name.as_str()),
            PageType::PostPage { post } => (format!("{} - {}", post.title, CONFIG.blog_name.as_str())),
          }
        }
        // link rel="stylesheet" type="text/css" href="/static/style.css";
        // link rel="shorcut icon" type="image/png" href="/static/favicon.png";
      }
      body id={
        @match &page {
          PageType::HomePage { .. } => ("homepage"),
          PageType::PostPage { .. } => ("post"),
        }
      } {
        header {
          h1 #blog-title dir="auto" { a href={"/"} { (CONFIG.blog_name.as_str()) } }
          @match &page {
            PageType::HomePage { .. } => {
              p.description { (CONFIG.description.as_str()) }
            },
            _ =>  {},
          }
          nav {}
        }
        @match &page {
          PageType::HomePage { posts, current_page } => {
            section #wrapper {
              @for post in posts.iter().filter(|p| p.published) {
                article.preview-post {
                  h2.post-title { a href=(format!("/{}", post.path)) { (PreEscaped(&post.title)) } }
                  time.created datetime={(post.format_time(TimeType::Created, false))} { (post.format_time(TimeType::Created, true)) }
                  div { (PreEscaped(&post.preview)) }
                  a #read-more href=(format!("/{}", post.path)) { "Read more" }
                }
              }
              @let total_pages = (POSTS.read().unwrap().len() as f32 / CONFIG.posts_per_page as f32).ceil() as usize;
              nav.content-container #paging {
                @if *current_page < total_pages {
                  a.older href={(format!("/page/{}", current_page + 1))} { "⇠ Older" }
                }
                @if *current_page > 1 {
                  a.newer href={(format!("/page/{}", current_page - 1))} { "Newer ⇢" }
                }
              }
            }
          },
          PageType::PostPage { post } => {
            article #post-body {
              h2 #title { (PreEscaped(&post.title)) }
              div.time {
                time.created datetime=(post.format_time(TimeType::Created, false)) { (post.format_time(TimeType::Created, true)) }
                @if post.time_diff_minutes() > 5.0 {
                  time.modified datetime=(post.format_time(TimeType::Modified, false)) { "Updated: " (post.format_time(TimeType::Modified, true)) }
                }
              }
              div { (PreEscaped(&post.content)) }
            }
          },
        }
        footer {
          hr;
          nav {
            p { "© 2024 " (CONFIG.author) }
          }
        }
      }
    }
  }
}
