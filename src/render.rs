use maud::{html, Markup, PreEscaped};

use crate::config::{BLOG_NAME, DESCRIPTION, POSTS_PER_PAGE};
use crate::post::Post;


pub fn render_homepage(posts: &[Post], current_page: usize, total_posts: usize) -> Markup {
  let total_pages = (total_posts as f32 / POSTS_PER_PAGE as f32).ceil() as usize;
  html! {
    html {
      head {
        title { (BLOG_NAME.as_str()) }
      }
      body {
        header {
          h1 { (BLOG_NAME.as_str()) }
          p { (DESCRIPTION.as_str()) }
          nav {}
        }
        section {
          @for post in posts {
            article {
              h2 {
                a href=(format!("/{}", post.path)) { (PreEscaped(&post.title)) }
              }
              time { (post.date.format("%d %B %Y, %H:%M")) }
              div { (PreEscaped(&post.preview)) }
              a href=(format!("/{}", post.path)) { "Read more" }
            }
          }
          nav {
            @if current_page < total_pages {
              a href={(format!("/page/{}", current_page + 1))} { "Older" }
            }
            " | "
            @if current_page > 1 {
              a href={(format!("/page/{}", current_page - 1))} { "Newer" }
            }
          }
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
}

pub fn render_post(post: &Post) -> Markup {
  html! {
    html {
      head {
        title { (format!("{} - {}", post.title, BLOG_NAME.as_str())) }
      }
      body {
        header {
          h1 {
            a href={"/"} { (BLOG_NAME.as_str()) }
          }
          nav {}
        }
        article {
          h2 { (PreEscaped(&post.title)) }
          time { (post.date.format("%d %B %Y, %H:%M")) }
          div { (PreEscaped(&post.content)) }
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
}
