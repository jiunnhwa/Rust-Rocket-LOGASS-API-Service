#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use chrono::Utc;
use rocket::*;
use rocket_contrib::json::Json;
use rocket::Request;
use rusqlite::{params, Connection, Result};
use serde::Serialize;
use serde::Deserialize;

use crate::sqlite::{ create_db, insert_sample_data};

mod sqlite;

//Prepare options
fn rocket() -> rocket::Rocket {
  rocket::ignite()
  .register(catchers![internal_error,not_found])
  .mount(
      "/",
      routes![index, get_logs,post_log,delete_log],
  )
}

//Main entry point
fn main() {
  println!("Welcome to Rust-Logass(Logging-As-A-Service)!");

  //ensure db
  create_db(DB);
  insert_sample_data(DB);

  //launch prepared rocket
  rocket().launch();  
}

//Home page
#[get("/")]
fn index() -> &'static str {
    "Welcome to Rust-Logass(Logging-As-A-Service)!"
}

//List of log items
#[derive(Serialize)]
struct LogItemList {
    items: Vec<LogItem>,
}

//log record
#[derive(Serialize,Deserialize)]
struct LogItem {
    RID: i64,
    UnixTime: i64,
    LogText: String,    
}

//json message
#[derive(Serialize)]
struct StatusMessage {
    message: String,
}

const DB: &str = "log.db";

//show all entries
#[get("/logs")]
fn get_logs() -> Result<Json<LogItemList>, String> {
    let db_connection = match Connection::open(DB) {
        Ok(connection) => connection,
        Err(_) => {return Err(String::from("Error connecting to database"));}
    };

    let mut statement = match db_connection.prepare("SELECT RID, UnixTime, LogText FROM Log;") {
        Ok(statement) => statement,
        Err(_) => return Err("Error preparing query".into()),
    };

    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(LogItem {
            RID: row.get(0)?,
            UnixTime: row.get(1)?,
            LogText: row.get(2)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            match collection {
                Ok(items) => Ok(Json(LogItemList { items })),
                Err(_) => Err("Error collecting items".into()),
            }
        }
        Err(_) => Err("Error fetching items".into()),
    }
}

//post an item
#[post("/log", format = "json", data = "<LogText>")]
fn post_log(LogText: Json<String>) -> Result<Json<StatusMessage>, String> {  
    let db_connection = match Connection::open(DB) {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Error connecting to database."));
        }
    };

    let mut statement =
        match db_connection.prepare("INSERT INTO Log (RID, UnixTime, LogText)  VALUES (NULL, ?1, ?2);") {
            Ok(statement) => statement,
            Err(_) => return Err("Error preparing query.".into()),
        };
        let item = LogItem {
          RID: 0,
          UnixTime: Utc::now().timestamp(),
          LogText: LogText.to_string(),
      };

    let results = statement.execute(params![item.UnixTime, item.LogText]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {message: format!("{} rows inserted.", rows_affected),})),
        Err(_) => Err("Error inserting item".into()),
    }
}

//delete a record
#[delete("/log/<id>")]
fn delete_log(id: i64) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open(DB) {
        Ok(connection) => connection,
        Err(_) => { return Err(String::from("Error connecting to database."));}
    };

    let mut statement = match db_connection.prepare("DELETE FROM LOG WHERE RID = $1;") {
        Ok(statement) => statement,
        Err(_) => return Err("Error preparing query".into()),
    };
    let results = statement.execute(&[&id]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {message: format!("{} row(s) deleted.", rows_affected),})),
        Err(_) => Err("Error deleting item".into()),
    }
}

//custom 500
#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

//custom 404
#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

