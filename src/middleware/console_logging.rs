use axum::response::{Response};

pub async fn log_api_request_to_console(req:Request) -> Request
{
    println!("[INFO]: {:?}", req);

    return req;
}

pub async fn log_api_response_to_console(res:Response) -> Response
{
    println!("[INFO]: {:?}", res);

    return res;
}