use std::{collections::HashMap, fs};
use std::sync::{Arc, RwLock, LazyLock};
use comrak::{markdown_to_html, ComrakOptions};
use time::{format_description, OffsetDateTime};


pub static POSTS: LazyLock<Arc<RwLock<Vec<Post>>>> = LazyLock::new(|| {
  Arc::new(RwLock::new(Post::load_posts()))
});


pub fn reload_posts() {
  let mut posts = POSTS.write().unwrap();
  *posts = Post::load_posts();
}


pub enum TimeType {
  Created,
  Modified,
}


#[derive(Debug)]
pub struct Post {
  pub created_time: OffsetDateTime,
  pub modified_time: OffsetDateTime,
  pub title: String,
  pub preview: String,
  pub content: String,
  pub path: String,
  pub published: bool,
}

impl Post {
  fn load_posts() -> Vec<Post> {
    let mut posts: Vec<Post> = Vec::new();
    let mut paths = HashMap::new();

    let posts_dir = fs::read_dir("posts").expect("Failed to read posts dir");
    for entry in posts_dir {
      let path = entry.unwrap().path();
      let ext = path.extension().and_then(|ext| ext.to_str());
      if path.is_file() && ext == Some("md") {
        if let Some(post) = Post::from_markdown(path.to_str().unwrap()){
          posts.push(post);
        }
      }
    }

    posts.sort_by(|a, b| b.created_time.cmp(&a.created_time));
    for post in posts.iter_mut().rev() {
      if let Some(count) = paths.get_mut(&post.path) {
        *count += 1;
        post.path = format!("{}-{}", post.path, count);
      } else {
        paths.insert(post.path.clone(), 0);
      }
    }

    posts
  }

  pub fn from_markdown(file_name: &str) -> Option<Self> {
    let raw_content = fs::read_to_string(file_name).unwrap();
    let parts: Vec<&str>= raw_content.splitn(3, "---\n").collect();
    if parts.len() != 3 {
      println!("The file {} is not in the correct format, skipping", file_name);
      return None;
    }

    let (title, created_time, modified_time, published) = Self::parse_metadata(parts[1]);
    let (preview, content) = Self::parse_content(parts[2]);
    let path = title.chars()
      .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
      .collect::<String>().to_lowercase().replace(' ', "-");

    Some(Post {
      created_time,
      modified_time,
      title,
      preview,
      content,
      path,
      published,
    })
  }

  pub fn format_time(&self, time_type: TimeType) -> String {
    let format = format_description::parse(
      "[day] [month repr:long] [year], [hour repr:24]:[minute]"
    ).unwrap();
    let ftime = match time_type {
      TimeType::Created => self.created_time,
      TimeType::Modified => self.modified_time,
    };
    ftime.format(&format).unwrap()
  }

  pub fn time_diff_minutes(&self) -> f32 {
    let duration = self.modified_time - self.created_time;
    duration.whole_seconds() as f32 / 60.0
  }

  fn adjust_headings(markdown: &str) -> String {
    markdown.lines()
      .map(|line| {
        if let Some(_stripped) = line.strip_prefix('#') {
          format!("#{}", line)
        } else {
          line.to_string()
        }
      }).collect::<Vec<_>>().join("\n")
  }

  fn parse_metadata(header: &str) -> (String, OffsetDateTime, OffsetDateTime, bool) {
    let metadata: HashMap<String, String> = header.lines().filter_map(|line| {
      let mut split = line.splitn(2, ": ");
      if let (Some(key), Some(val)) = (split.next(), split.next()) {
        Some((key.trim().to_string(), val.trim().to_string().replace("\"", "")))
      } else {
        None
      }
    }).collect();

    let title = metadata.get("title").filter(
      |t| !t.is_empty()
    ).cloned().unwrap_or_else(|| "Untitled".to_string());
    let created_time = metadata.get("created_time").and_then(
      |time_str| OffsetDateTime::parse(time_str, &format_description::well_known::Rfc3339).ok()
    ).unwrap_or_else(|| OffsetDateTime::now_utc());
    let modified_time = metadata.get("modified_time").and_then(
      |time_str| OffsetDateTime::parse(time_str, &format_description::well_known::Rfc3339).ok()
    ).unwrap_or_else(|| OffsetDateTime::now_utc());
    let published = metadata.get("published").map(|p| p == "true").unwrap_or(false);

    (title, created_time, modified_time, published)
  }

  fn parse_content(post_content: &str) -> (String, String) {
    let paragraphs: Vec<&str> = post_content.split("\n\n").collect();
    let preview_text = Self::adjust_headings(&paragraphs[..paragraphs.len().min(5)].join("\n\n"));
    let full_text = Self::adjust_headings(post_content);
    let preview_html = markdown_to_html(&preview_text, &ComrakOptions::default());
    let full_html = markdown_to_html(&full_text, &ComrakOptions::default());
    (preview_html, full_html)
  }
}
