use askama::Template;

use crate::middleware::auth::Ctx;

use super::{HeaderComponent, NavbarComponent, NavbarLink};

#[derive(Template)]
#[template(path = "web/index.html")]
pub struct Index<'a>
{
    header: HeaderComponent<'a>,
    navbar: NavbarComponent<'a>,
}

pub async fn index<'a>(ctx_option: Option<Ctx>) -> Index<'a>
{
    let navbar = if ctx_option.is_some()
    {
        NavbarComponent {
            button_value: "Log out",
            button_crud_type: "post",
            button_route: "/logout",
            links: vec![NavbarLink {
                value: "App",
                redirect: "/channels",
            }],
        }
    }
    else
    {
        NavbarComponent {
            button_value: "Log in",
            button_crud_type: "get",
            button_route: "/login",
            links: Vec::new(),
        }
    };

    Index {
        header: HeaderComponent::new("Index"),
        navbar,
    }
}
