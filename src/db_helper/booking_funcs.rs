use std::sync::MutexGuard;

use anyhow::{anyhow, Error};
use sqlite::Connection;



const GET_BOOKINGS: &str = "SELECT player_one, player_two, reference FROM Bookings WHERE date_string = ?";
const MAKE_BOOKING: &str = "INSERT INTO Bookings (player_one, player_two, reference, date_string) VALUES (?, ?, ?, ?)";
const REMOVE_BOOKING: &str = "DELETE FROM Bookings where player_one = ? AND player_two = ? AND date_string = ?";

pub fn get_bookings(
    db_con: MutexGuard<Connection>,
    day: &str
) -> Result<Vec<(i64, i64, Option<String>)>, Error> {
    let mut stmt = db_con.prepare(GET_BOOKINGS)?;
    stmt.bind((1, day))?;

    

    let mut bookings = Vec::new();
    for row in stmt.iter() {
        let row = row?;
        let p1 = row.read::<i64, _>(0);
        let p2 = row.read::<i64, _>(1);
        let reference_str = row.read::<Option<&str>, _>(2);
        let reference = if reference_str.is_some() {
            Some(reference_str.unwrap().to_string())
        } else {
            None
        };
        bookings.push((p1, p2, reference));
    }

    
    
    Ok(bookings)
}

pub fn book_game(
    db_con: MutexGuard<Connection>,
    day: &str,
    player_one: i64,
    player_two: i64,
    reference: Option<&str>
) -> Result<(), Error> {
    let mut stmt = db_con.prepare(MAKE_BOOKING)?;
    stmt.bind((1, player_one))?;
    stmt.bind((2, player_two))?;
    stmt.bind((3, reference))?;
    stmt.bind((4, day))?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("Error while adding game"))
    }
}

pub fn remove_game(
    db_con: MutexGuard<Connection>,
    day: &str,
    player_one: i64,
    player_two: i64,
) -> Result<(), Error> {
    let mut stmt = db_con.prepare(REMOVE_BOOKING)?;
    stmt.bind((1, player_one))?;
    stmt.bind((2, player_two))?;
    stmt.bind((3, day))?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("Error while removing game"))
    }
}