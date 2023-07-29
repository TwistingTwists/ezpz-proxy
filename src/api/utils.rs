use axum::{response::Response, body::Body};
use http::StatusCode;
use serde_json::Value;

pub fn reponse_json(data: Value, status: StatusCode) -> Response<Body> {
  let mut res = Response::builder();
  res = res.header("Content-Type", "application/json");
  res = res.status(status);
  return res.body(Body::from(data.to_string())).unwrap();
}


#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  #[test]
  fn test_reponse_json() {
    let data = json!({"name": "test"});
    let res = reponse_json(data, StatusCode::OK);
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.headers().get("Content-Type").unwrap(), "application/json");
    let (_, body) = res.into_parts();
    tokio::runtime::Runtime::new().unwrap().block_on(async {
      let body = hyper::body::to_bytes(body).await.unwrap();
      assert_eq!(body, "{\"name\":\"test\"}".as_bytes());
    });
  }
}
