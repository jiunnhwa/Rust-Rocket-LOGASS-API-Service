#![allow(dead_code)]
use rusqlite::{params, Connection};
use rusqlite::NO_PARAMS;
use chrono::Utc;

//Create the db and table if not exists
pub fn create_db(db: &str)  {
    let conn = Connection::open(db).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Log (
             RID integer primary key,
             UnixTime integer,  
             LogText text not null
         )",
        NO_PARAMS,
    ).unwrap();

}

//Seed sample data to db
pub fn insert_sample_data(db: &str)  {
    struct Log {
        UnixTime: i64,
        LogText: String,
    }
    
    let conn = Connection::open(db).unwrap();
    let item = Log {
        UnixTime: Utc::now().timestamp(),
        LogText: "Sample log item".to_string(),
    };

    conn.execute(
        "INSERT INTO Log (RID, UnixTime, LogText) VALUES (NULL, ?1, ?2)",
        params![item.UnixTime, item.LogText],
    ).unwrap();

}
