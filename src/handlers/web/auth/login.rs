use axum::response::Html;

pub async fn login() -> Html<String> 
{
    Html("<h1>Hello there test</h1>".to_string())
}