use std::{collections::HashMap, convert::Infallible, net::SocketAddr, path::Path};

use axum::{
    body::{Body, BoxBody},
    http::{Request, Response, StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router,
};
use once_cell::sync::Lazy;
use tower::ServiceExt;
use tower_http::services::ServeDir;

static VERB_DIR: Lazy<HashMap<&str, &str>> =
    Lazy::new(|| HashMap::from([("pouet", "/home/geobert/tmp/tower-servedir-test/www/")]));

#[tokio::main]
async fn main() {
    let router = Router::new().fallback_service(get(serve_file));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    dbg!(
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
    )
    .unwrap();
}

async fn serve_file(url: Uri, req: Request<Body>) -> Result<Response<BoxBody>, Infallible> {
    // I use `url` to determine which directory to serve, as it is configured somewhere that can be updated
    dbg!(&url);
    let uri = &url.path()[1..];
    dbg!(uri);
    if let Some(&verb) = VERB_DIR.get(uri) {
        let path = Path::new(verb);

        // read to prove it exists
        dbg!(&std::fs::read_to_string(path.join("index.html")));

        let service = ServeDir::new(path);
        Ok(service.oneshot(req).await.into_response())
    } else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }
}
