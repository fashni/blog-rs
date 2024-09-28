use std::{ffi::OsStr, fs, sync::LazyLock};
use comrak::{markdown_to_html, ComrakOptions};
use time::{format_description, OffsetDateTime};


pub static POSTS: LazyLock<Vec<Post>> = LazyLock::new(|| {
  let mut posts: Vec<Post> = Vec::new();
  for entry in fs::read_dir("posts").unwrap() {
    let path = entry.unwrap().path();
    if path.is_file() && path.extension().unwrap_or(OsStr::new("")) == "md" {
      let post = Post::from_markdown(path.to_str().unwrap());
      posts.push(post);
    }
  }
  posts.sort_by(|a, b| b.created_time.cmp(&a.created_time));
  posts
});

pub enum TimeType {
  Created,
  Modified,
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


#[derive(Debug)]
pub struct Post {
  pub created_time: OffsetDateTime,
  pub modified_time: OffsetDateTime,
  pub title: String,
  pub preview: String,
  pub content: String,
  pub path: String,
}

impl Post {
  pub fn from_markdown(file_name: &str) -> Self {
    let content = fs::read_to_string(file_name).unwrap();
    let metadata = fs::metadata(file_name).unwrap();

    let modified_time: OffsetDateTime = metadata.modified().unwrap().into();
    let created_time: OffsetDateTime = metadata.created().unwrap().into();

    let mut lines = content.lines();
    let title = lines.next().unwrap().trim_start_matches("# ").trim_end().to_string();
    let path = title.chars()
      .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
      .collect::<String>().to_lowercase().replace(' ', "-");

    let content: Vec<&str> = lines.collect();
    let full_text = content.join("\n\n");
    let full_text = adjust_headings(&full_text);
    let full_html = markdown_to_html(&full_text, &ComrakOptions::default());

    let preview_text = content[..content.len().min(5)].join("\n\n");
    let preview_text = adjust_headings(&preview_text);
    let preview_html = markdown_to_html(&preview_text, &ComrakOptions::default());

    Post {
      created_time,
      modified_time,
      title,
      preview: preview_html,
      content: full_html,
      path,
    }
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
}
