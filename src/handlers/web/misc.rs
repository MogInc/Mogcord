use askama::Template;

use crate::middleware::auth::Ctx;

use super::{HeadComponent, HeaderComponent, NavLink};

#[derive(Template)]
#[template(path = "web/index.html")]
pub struct Index<'a>
{
    head: HeadComponent<'a>,
    header: HeaderComponent<'a>,
}

pub async fn index<'a>(ctx_option: Option<Ctx>) -> Index<'a>
{
    let header = if ctx_option.is_some()
    {
        HeaderComponent {
            button_value: "Log out",
            button_crud_type: "post",
            button_route: "/logout",
            links: vec![NavLink {
                value: "App",
                redirect: "/channels",
            }],
        }
    }
    else
    {
        HeaderComponent {
            button_value: "Log in",
            button_crud_type: "get",
            button_route: "/login",
            links: Vec::new(),
        }
    };

    Index {
        head: HeadComponent::new("Index"),
        header,
    }
}
