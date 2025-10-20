use std::sync::MutexGuard;

use anyhow::{anyhow, Error};
use sqlite::Connection;



const ADD_DAY: &str = "INSERT INTO Days (date_string) VALUES (?)";

pub fn add_day(
    db_con: MutexGuard<Connection>,
    day: &str
) -> Result<(), Error> {

    let mut stmt = db_con.prepare(ADD_DAY)?;
    stmt.bind((1, day))?;

    if stmt.next()? == sqlite::State::Done {
        Ok(())
    } else {
        Err(anyhow!("error while inserting day"))
    }
}
