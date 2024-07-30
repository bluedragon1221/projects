use list_comprehension_macro::comp;
use polars::df;
use polars::frame::DataFrame;
use rocket::State;
use rocket::{
    form::Form, fs::FileServer, get, launch, post, response::Redirect, routes, uri, FromForm
};
use rocket_db_pools::{Connection, Database};
use rocket_dyn_templates::{context, Template};
use serde::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::sync::RwLock;

const DATA_FILE: &str = "static/game.csv";

#[derive(Database)]
#[database("game_data")]
struct GameDB(sqlx::SqlitePool);

#[derive(Default)]
struct GameData {
    players: Vec<String>,
    game: Vec<Vec<i32>>,
}

// fn get_data() -> io::Result<GameData> {
//     let content = fs::read_to_string(DATA_FILE)?;
//     let mut lines = content.lines();
    
//     let players: Vec<String> = lines
//         .next()
//         .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing player data"))?
//         .split(',')
//         .map(String::from)
//         .collect();

//     let game: Vec<Vec<i32>> = lines
//         .map(|line| {
//             line.split(',')
//                 .map(|x| x.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)))
//                 .collect::<Result<Vec<i32>, _>>()
//         })
//         .collect::<Result<Vec<Vec<i32>>, _>>()?;

//     Ok(GameData { players, game })
// }

#[get("/")]
fn game_tracker(state: &State<RwLock<DataFrame>>) -> Template {
    let df = state.inner().read().unwrap();
    let total_scores = df.lazy().sum().unwrap().to_vec::<i32>();
    Template::render(
        "game_tracker",
        context! {
            game: &data.game,
            players: df.inner().get_column_names(),
            total_scores: df.inner(),
        },
    )
}

#[post("/export-csv")]
fn export_csv() -> io::Result<String> {
    fs::read_to_string(DATA_FILE)
}

#[derive(Deserialize, FromForm)]
struct AddRoundData {
    scores: Vec<i32>,
}

#[post("/add-round", data = "<form>")]
fn add_round(form: Form<AddRoundData>) -> io::Result<Redirect> {
    let scores = form.into_inner().scores;
    let mut file = fs::OpenOptions::new().append(true).open(DATA_FILE)?;
    writeln!(file, "{}", scores.iter().map(ToString::to_string).collect::<Vec<_>>().join(","))?;
    Ok(Redirect::to(uri!("/")))
}

#[post("/game-graph")]
fn generate_graph() -> &'static str {
    "Graph generation not implemented yet"
}

#[launch]
fn rocket() -> _ {
    let df = df![
        "Collin" => &[None::<i32>; 0],
        "Brennan" => &[None::<i32>; 0],
        "Ethan" => &[None::<i32>; 0],
        "Katelyn" => &[None::<i32>; 0],
        "Mom" => &[None::<i32>; 0],
        "Dad" => &[None::<i32>; 0],
    ];

    rocket::build()
        .mount(
            "/",
            routes![game_tracker, add_round, export_csv, generate_graph],
        )
        .mount("/public", FileServer::from("static"))
        .attach(Template::fairing())
        .manage(RwLock::new(df))
}