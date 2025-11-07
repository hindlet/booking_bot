use std::{path::PathBuf, sync::MutexGuard};
use sqlite::{Connection};

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

pub fn init_database(db_con: MutexGuard<Connection>) {
    let init_query = "

    CREATE TABLE IF NOT EXISTS Days (
        id INTEGER PRIMARY KEY,
        date_string TEXT NOT NULL,
        UNIQUE (date_string) ON CONFLICT ABORT
    );

    CREATE TABLE IF NOT EXISTS Bookings (
        id INTEGER PRIMARY KEY,
        player_one INTEGER NOT NULL,
        player_two INTEGER NOT NULL,
        reference TEXT,
        date_string TEXT NOT NULL,
        FOREIGN KEY(date_string) REFERENCES Days(date_string),
        UNIQUE(player_one, player_two, date_string) ON CONFLICT ABORT
    );
    ";

    db_con
        .execute(init_query)
        .expect("Database Initialised");
}



