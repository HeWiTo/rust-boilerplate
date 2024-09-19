use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub title: &'a str,
    pub user: Option<User<'a>>,
}

pub struct User<'a> {
    pub name: &'a str,
}