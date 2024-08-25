use askama::Template;

use crate::middleware::auth::Ctx;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index<'a>
{
    title: &'a str,
    nav_button_value: &'a str,
    nav_button_crud_type: &'a str,
    nav_button_route: &'a str,
}

pub async fn index<'a>(ctx_option: Option<Ctx>) -> Index<'a>
{
    let (nav_button_value, nav_button_crud_type, nav_button_route) =
        if ctx_option.is_some()
        {
            ("Log out", "post", "/logout")
        }
        else
        {
            ("Log in", "get", "/login")
        };

    Index {
        title: "Index",
        nav_button_value,
        nav_button_crud_type,
        nav_button_route,
    }
}
