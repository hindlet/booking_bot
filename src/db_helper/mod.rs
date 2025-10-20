use std::path::PathBuf;
use sqlite::{Connection};
use anyhow::{anyhow, Error};

pub mod day_funcs;
pub mod booking_funcs;



pub fn open_database(path: PathBuf) -> Connection {
    let db_path = {
        let mut p = path.into_os_string();
        p.push("/bookings.db");
        p
    };
    sqlite::open(db_path).expect("Failed to create sqlite database")
}

pub fn init_database(db_con: Connection) {
    let init_query = "

    CREATE TABLE IF NOT EXISTS Days (
        id INTEGER PRIMARY KEY NOT NULL,
        date TEXT NOT NULL,
        UNIQUE date ON CONFLICT ABORT
    );

    CREATE TABLE IF NOT EXISTS Bookings (
        id INTEGER PRIMARY KEY NOT NULL,
        player_one TEXT NOT NULL,
        player_one_id INTEGER NOT NULL,
        player_two TEXT NOT NULL,
        player_two_id INTEGER NOT NULL,
        FOREIGN KEY(date_id) REFERENCES Days(id)
        reference TEXT
        UNIQUE(player_one_id, player_two_id, date_id) ON CONFLICT ABORT
    );
    ";

    db_con
        .execute(init_query)
        .expect("Database Initialised");
}



