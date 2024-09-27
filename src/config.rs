use once_cell::sync::Lazy;


pub const PORT: usize = 8000;
pub const POSTS_PER_PAGE: usize = 5;
pub static BLOG_NAME: Lazy<String> = Lazy::new(|| String::from("The Murky Lens"));
pub static DESCRIPTION: Lazy<String> = Lazy::new(|| String::from("Finding some clarity in the maze of self-expression through a murky lens"));
