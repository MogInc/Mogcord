use askama::Template;

use crate::middleware::auth::Ctx;

use super::{NavbarComponent, NavbarLink};

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct Index<'a>
{
    title: &'a str,
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
        title: "Index",
        navbar,
    }
}
