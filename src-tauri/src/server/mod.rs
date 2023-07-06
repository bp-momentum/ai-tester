use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde_json::{json, Value};
use std::convert::Infallible;
use tokio::sync::oneshot::Receiver;

use crate::events::send_persistent_feedback;
use crate::{APP_HANDLE, LANDMARKS};

async fn landmarks(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let landmarks = LANDMARKS.lock().unwrap();
    if landmarks.is_none() {
        let response = json!({
          "success": false,
          "description": "No landmarks set"
        });
        return Ok(Response::new(
            serde_json::to_string(&response).unwrap().into(),
        ));
    }

    let lmrks: Value = serde_json::from_str(landmarks.as_ref().unwrap().as_str()).unwrap();

    drop(landmarks);

    let response = json!({
      "success": true,
      "description": "Returned data",
      "data": {
        "title": "Test Exercise",
        "description": "This is a test exercise",
        "video": "https://www.youtube.com/watch?v=9bZkp7q19f0",
        "expectation": lmrks
      }
    });
    Ok(Response::new(
        serde_json::to_string(&response).unwrap().into(),
    ))
}

async fn feedback(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body = hyper::body::to_bytes(_req.into_body()).await.unwrap();
    let body = String::from_utf8(body.to_vec()).unwrap();
    let handle = APP_HANDLE.lock().unwrap();

    let Ok(json) = serde_json::from_str::<Value>(&body) else {
    let response = json!({
      "success": false,
      "description": "Invalid JSON"
    });
    let res = Response::new(serde_json::to_string(&response).unwrap().into());
    return Ok(res);
  };

    send_persistent_feedback(handle.as_ref().unwrap(), json);
    let response = json!({
      "success": true,
      "description": "Feedback received"
    });
    let res = Response::new(serde_json::to_string(&response).unwrap().into());
    Ok(res)
}

async fn handle_req(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // path = /api/internal/rate
    if _req.uri().path().starts_with("/api/getexercise") {
        return landmarks(_req).await;
    }

    // path = /api/getexercise
    if _req.uri().path() == "/api/internal/rate" {
        return feedback(_req).await;
    }
    Ok(Response::new(Body::from("")))
}

#[tokio::main]
pub async fn spawn_tokio(
    port: u16,
    rx: Receiver<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(handle_req)) }
    });

    let addr = ([0, 0, 0, 0], port).into();

    let server = Server::bind(&addr).serve(make_svc);

    server
        .with_graceful_shutdown(async {
            rx.await.ok();
        })
        .await?;

    Ok(())
}
