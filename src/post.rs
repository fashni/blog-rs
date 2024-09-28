use std::{ffi::OsStr, fs};
use chrono::{NaiveDateTime, Utc};
use comrak::{markdown_to_html, ComrakOptions};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};


pub static POSTS: Lazy<Vec<Post>> = Lazy::new(|| {
  let mut posts: Vec<Post> = Vec::new();
  for entry in fs::read_dir("posts").unwrap() {
    let path = entry.unwrap().path();
    if path.is_file() && path.extension().unwrap_or(OsStr::new("")) == "md" {
      let post = Post::from_markdown(path.to_str().unwrap());
      posts.push(post);
    }
  }
  posts.sort_by(|a, b| b.date.cmp(&a.date));
  posts
});

fn adjust_headings(markdown: &str) -> String {
  let re = Regex::new(r"(?m)^(#+\s)").unwrap();
  re.replace_all(markdown, |caps: &Captures| {
    let pounds = &caps[1];
    format!("#{}", pounds)
  }).to_string()
}


#[derive(Debug)]
pub struct Post {
  pub date: NaiveDateTime,
  pub title: String,
  pub preview: String,
  pub content: String,
  pub path: String,
}

impl Post {
  pub fn from_markdown(file_name: &str) -> Self {
    fn get_path(title: &str) -> String {
      let re = Regex::new(r"[^\w\s\-]").unwrap();
      let path = re.replace_all(title, "").to_string();
      path.to_lowercase().replace(' ', "-")
    }

    let content = fs::read_to_string(file_name).expect(&format!("Failed to load file: {}", file_name)[..]);
    let mut lines = content.lines();

    let date = lines.next().unwrap();
    let date = NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S%.f")
      .unwrap_or_else(
      |_| Utc::now().naive_utc()
    );

    let title = lines.next().unwrap().trim_start_matches("# ").trim_end().to_string();
    let path = get_path(&title);

    let content: Vec<&str> = lines.collect();
    let full_text = content.join("\n\n");
    let full_text = adjust_headings(&full_text);
    let full_html = markdown_to_html(&full_text, &ComrakOptions::default());

    let preview_text = content[..content.len().min(5)].join("\n\n");
    let preview_text = adjust_headings(&preview_text);
    let preview_html = markdown_to_html(&preview_text, &ComrakOptions::default());

    Post {
      date,
      title,
      preview: preview_html,
      content: full_html,
      path,
    }
  }
}
