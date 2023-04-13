use std::num::ParseIntError;

use lambda_http::{Request, RequestExt};

pub fn pagination_parameters(event: &Request) -> Result<(u64, u64), ParseIntError> {
    let page_size = event
        .query_string_parameters()
        .first("page_size")
        .unwrap_or("50")
        .parse::<u64>()?;
    let page = event
        .query_string_parameters()
        .first("page")
        .unwrap_or("0")
        .parse::<u64>()?;
    Ok((page_size, page))
}
