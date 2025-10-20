use std::sync::MutexGuard;

use anyhow::{anyhow, Error};
use sqlite::Connection;



const GET_BOOKINGS: &str = "SELECT player_one, player_two, reference FROM Bookings WHERE date_id = ?";

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