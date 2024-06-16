use axum::{response::Response};
use serde::{Serialize, Deserialize};
use serde_json;

pub async fn log_api_route_to_console(res:Response) -> Response
{
    let j = serde_json::to_string(&res);

    println!("[INFO]: {}", res.status());

    return res;
}