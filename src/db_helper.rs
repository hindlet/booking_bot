use std::path::PathBuf;

use sqlite::Connection;



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
        description TEXT NOT NULL,
    );

    CREATE TABLE IF NOT EXISTS Bookings (
        id INTEGER PRIMARY KEY NOT NULL,
        player_one TEXT NOT NULL,
        player_two TEXT NOT NULL,
        FOREIGN KEY(day_id) REFERENCES Days(id)
        reference TEXT
        UNIQUE(player_one_id, player_two_id, day_id) ON CONFLICT ABORT
    );
    ";

    db_con
        .execute(init_query)
        .expect("Database Initialised");
}

pub fn add_day(

) {

}
