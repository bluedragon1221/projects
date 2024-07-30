use rocket::{form::Form, get, launch, post, response::Redirect, routes, uri, FromForm};
use rocket_db_pools::{sqlx, Connection, Database};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

#[derive(Database)]
#[database("message_logs")]
struct MessageLogs(sqlx::SqlitePool);

#[derive(Serialize)]
struct Link {
    text: String,
    href: String,
}

#[derive(FromForm, Serialize, Deserialize, Debug, sqlx::FromRow)]
struct Message {
    username: String,
    message: String,
}

#[post("/", data = "<message_data>")]
async fn say_hello(mut db: Connection<MessageLogs>, message_data: Form<Message>) -> Redirect {
    sqlx::query!("INSERT INTO messages (username, message) VALUES (?, ?)", message_data.username, message_data.message)
        .execute(&mut **db)
        .await
        .unwrap();

    Redirect::to(uri!("/"))
}

#[get("/")]
async fn main_window(mut db: Connection<MessageLogs>) -> Option<Template> {
    // Make sure the messages table exists
    // Ideally it would just run on page load, but I don't know how to do that
    sqlx::query!("CREATE TABLE IF NOT EXISTS messages (username TEXT NOT NULL, message TEXT NOT NULL)")
        .execute(&mut **db)
        .await.unwrap();

    let messages: Vec<Message> =
        sqlx::query_as!(Message, "SELECT username, message from messages")
            .fetch_all(&mut **db)
            .await
            .unwrap();

    let say_hello_link = Link {
        text: String::from("Say hello to Collin"),
        href: String::from("/say-hello/Collin"),
    };

    Some(Template::render(
        "main_window",
        context! {
            title: String::from("Chat App"),
            sidebar_links: vec![say_hello_link],
            messages: messages
        },
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![main_window, say_hello])
        .attach(MessageLogs::init())
        .attach(Template::fairing())
}
