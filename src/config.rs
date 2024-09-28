use std::sync::LazyLock;


pub const PORT: usize = 8000;
pub const POSTS_PER_PAGE: usize = 5;
pub static BLOG_NAME: LazyLock<String> = LazyLock::new(|| String::from("The Murky Lens"));
pub static DESCRIPTION: LazyLock<String> = LazyLock::new(|| String::from("Finding some clarity in the maze of self-expression through a murky lens"));
