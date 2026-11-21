#![forbid(unsafe_code)]
//! Executable server entry point for `kennedy.mosoti.dev`.

use site_content::{load_from_dir, validate_all};
use site_server::build_router;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process;

#[tokio::main]
async fn main() {
    let content_dir =
        PathBuf::from(std::env::var("SITE_CONTENT_DIR").unwrap_or_else(|_| "content".to_owned()));
    let public_dir =
        PathBuf::from(std::env::var("SITE_PUBLIC_DIR").unwrap_or_else(|_| "public".to_owned()));
    let pkg_dir = PathBuf::from(
        std::env::var("SITE_PKG_DIR").unwrap_or_else(|_| "target/site/pkg".to_owned()),
    );
    let addr: SocketAddr = std::env::var("SITE_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_owned())
        .parse()
        .expect("invalid SITE_ADDR socket address");

    let content = match load_from_dir(&content_dir) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("content load failed: {err}");
            process::exit(2);
        }
    };
    if let Err(errs) = validate_all(&content) {
        eprintln!("content validation failed:\n{}", errs.join("\n"));
        process::exit(2);
    }

    let app = build_router(content, public_dir, pkg_dir);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind SITE_ADDR");
    println!("kennedy-mosoti-dev listening on http://{addr}");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("server terminated with error");
}
