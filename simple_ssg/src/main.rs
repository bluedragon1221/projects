use std::fs;

use fronma::parser::parse as fronma_parse;
use markdown::to_html as md_to_html;
use rocket::{fs::{relative, FileServer}, get, launch, response::status::NotFound, routes, State};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
enum SsgError {
    #[error("This file does not exist: {0}")]
    FileNotFound(#[from] std::io::Error),

    #[error("There was an error while parsing frontmatter: {0}")]
    FrontmatterParserError(String),
}

impl From<fronma::error::Error> for SsgError {
    fn from(value: fronma::error::Error) -> Self {
        Self::FrontmatterParserError(format!("{value:?}"))
    }
}

#[derive(Serialize, Deserialize)]
struct ArticleFrontmatter {
    title: String,
    slug: String,
    author: String,
    summary: String,
    tags: Vec<String>,
}

struct Article {
    frontmatter: ArticleFrontmatter,
    content: String,
}

impl Article {
    fn from_file(path: &str) -> Result<Self, SsgError> {
        let file_text = fs::read_to_string(path)?;
        let parsed_file = fronma_parse::<ArticleFrontmatter>(&file_text)?;
        let content = md_to_html(parsed_file.body);
        let frontmatter = parsed_file.headers;

        Ok(Self {
            frontmatter,
            content,
        })
    }
}

#[derive(Default)]
struct ArticleList {
    title: String,
    articles: Vec<Article>,
}

impl ArticleList {
    fn from_article_list(articles: Vec<Article>, title: &str) -> Self {
        Self {
            title: title.into(),
            articles,
        }
    }

    fn get_by_slug(&self, slug: &str) -> Option<&Article> {
        self.articles.iter().find(|a| a.frontmatter.slug == slug)
    }
}

#[get("/")]
fn article_list(article_list: &State<ArticleList>) -> Template {
    let template_data: Vec<&ArticleFrontmatter> = article_list
        .articles
        .iter()
        .map(|x| &x.frontmatter)
        .collect();

    Template::render(
        "article_list",
        context! {
            title: &article_list.title,
            articles: template_data
        },
    )
}

#[get("/<slug>")]
fn get_slug(slug: &str, article_list: &State<ArticleList>) -> Result<Template, NotFound<String>> {
    if let Some(article) = article_list.get_by_slug(slug) {
        Ok(Template::render(
            "article",
            context! {
                name: &article.frontmatter.title,
                author: &article.frontmatter.author,
                content: &article.content,
            },
        ))
    } else {
        Err(NotFound(format!("Article not found: {slug}")))
    }
}

fn build_article_list() -> Result<ArticleList, SsgError> {
    let mut article_vec = Vec::<Article>::new();

    let articles_dir = fs::read_dir("../articles/")?;
    for file in articles_dir {
        article_vec
            .push(Article::from_file(&file?.path().display().to_string())?);
    }

    Ok(ArticleList::from_article_list(article_vec, "All Articles"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(build_article_list().unwrap_or_default())
        .mount("/", routes![article_list, get_slug])
        // .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
