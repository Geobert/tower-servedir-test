use std::{convert::Infallible, net::SocketAddr, path::Path};

use axum::{
    body::{Body, BoxBody},
    extract,
    http::{Request, Response},
    response::IntoResponse,
    routing::get,
    Router,
};
use tower::ServiceExt;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/*path", get(serve_file));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    dbg!(
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
    )
    .unwrap();
}

async fn serve_file(
    extract::Path(url): extract::Path<String>,
    req: Request<Body>,
) -> Result<Response<BoxBody>, Infallible> {
    // I use `url` to determine which directory to serve, as it is configured somewhere that can be updated
    dbg!(&url);

    let path = Path::new("/home/geobert/tmp/tower-servedir-test/www");

    // read to prove it exists
    dbg!(&std::fs::read_to_string(path.join("index.html")));

    let service = ServeDir::new(path);
    Ok(service.oneshot(req).await.into_response())
}
