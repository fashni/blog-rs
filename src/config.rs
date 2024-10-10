use std::sync::LazyLock;


pub static CONFIG: LazyLock<Config> = LazyLock::new(||
  Config::default()
);

pub struct Config {
  pub blog_name: String,
  pub description: String,
  pub author: String,
  pub port: u16,
  pub posts_per_page: usize,
}

impl Config {
  pub fn default() -> Self {
    Config {
      blog_name: String::from("The Murky Lens"),
      description: String::from("Finding some clarity in the maze of self-expression through a murky lens"),
      author: String::from("fashni"),
      port: 8000,
      posts_per_page: 5,
    }
  }
}
