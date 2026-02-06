use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Router, Json};
use kociemba::solver::solve;

fn app() -> Router {
    Router::new().route("/solve/{cube}", get(handler))
}

#[tokio::main]
async fn main() {
    println!("Running first solve.");
    println!("This may take a while if you haven't generated prune tables yet...");
    tokio::task::spawn_blocking(move || {
        solve(
            "RLLBUFUUUBDURRBBUBRLRRFDFDDLLLUDFLRRDDFRLFDBUBFFLBBDUF",
            20,
            3.0,).unwrap();
    }).await.unwrap();

    let app = app();

    println!("Starting server on port 3000...");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(axum::extract::Path(cube): axum::extract::Path<String>) -> impl IntoResponse {
    let timer = std::time::Instant::now();
    println!("Solving cube {}...", cube);
    match tokio::task::spawn_blocking(move || {solve(&cube, 21, 5.0)}).await {
        Ok(Ok(solution)) => {
            println!("Cube solved in {}ms.", timer.elapsed().as_millis());
            (StatusCode::OK, Json(solution)).into_response()
        },
        Ok(Err(e)) => {
            println!("Failed to solve after {}ms", timer.elapsed().as_millis());
            (
            StatusCode::BAD_REQUEST,
            [("x-error", "invalid cube string")],
            format!("Error: {:?}", e),
        )
            .into_response()},
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [("x-error", "thread join failed")],
            format!("Error: {:?}", e),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`, `ready`, and `call`
    use http_body_util::BodyExt; // for `collect`

    #[tokio::test]
    async fn solve_valid_cube() {
        let app = app();

        // A valid cube string
        let cube = "RLLBUFUUUBDURRBBUBRLRRFDFDDLLLUDFLRRDDFRLFDBUBFFLBBDUF";

        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/solve/{}", cube))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        // The solution is a list of moves, should be non-empty for this cube
        assert!(!body_str.is_empty());
    }

    #[tokio::test]
    async fn solve_invalid_cube() {
        let app = app();

        // An invalid cube string (too short)
        let cube = "INVALID";

        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/solve/{}", cube))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(response.headers().get("x-error").unwrap(), "invalid cube string");

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("Error:"));
    }
}
