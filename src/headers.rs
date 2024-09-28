use std::sync::LazyLock;
use tiny_http::Header;


pub static HTML_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "text/html; charset=UTF-8").unwrap());
pub static CSS_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "text/css").unwrap());
pub static JS_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "application/javascript").unwrap());
pub static PNG_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "image/png").unwrap());
pub static JPEG_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "image/jpeg").unwrap());
pub static GIF_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "image/gif").unwrap());
pub static SVG_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "image/svg+xml").unwrap());
pub static ICO_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "image/x-icon").unwrap());
pub static WOFF_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "font/woff").unwrap());
pub static WOFF2_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "font/woff2").unwrap());
pub static OCTET_STREAM_HEADER: LazyLock<Header> = LazyLock::new(|| Header::from_bytes("Content-Type", "application/octet-stream").unwrap());
