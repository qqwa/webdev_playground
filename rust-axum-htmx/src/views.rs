use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub username: &'a str,
    pub count: u32,
    pub github_id: i32,
}
