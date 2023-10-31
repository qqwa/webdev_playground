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

#[derive(Template, Default)]
#[template(path = "secret.html")]
pub struct SecretTemplate {
    pub username: String,
    pub box_template: BoxTemplate,
}

#[derive(Template, Default)]
#[template(path = "repos.html")]
pub struct ReposTemplate {
    pub repos: Vec<RepoInfo>,
}

pub struct RepoInfo {
    pub full_name: String,
    pub description: String,
    pub url: String,
}

#[derive(Template, Default)]
#[template(path = "box.html")]
pub struct BoxTemplate {
    pub color: String,
}
