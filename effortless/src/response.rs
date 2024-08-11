use lambda_http::{
    http::{header, StatusCode},
    Body, Error, Response,
};

pub fn make_json_created_response(body: String) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::Text(body))?)
}
