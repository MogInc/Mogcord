use axum::{extract::Path, routing::{get, post, Router}};

pub fn routes_user() -> Router
{
    Router::new()
    .route("/user/:id", get(get_user))
    .route("/user", post(post_user))
}

async fn get_user(Path(uuid): Path<String>) 
{
    println!("{}", uuid);
}

async fn post_user() 
{

}