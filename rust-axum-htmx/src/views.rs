use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub username: &'a str,
    pub count: u32,
    pub github_id: i32,
}

#[derive(Template, Default)]
#[template(path = "form.html")]
pub struct FormTemplate {
    pub response: Option<String>,
    pub csrf_token: String,
}
