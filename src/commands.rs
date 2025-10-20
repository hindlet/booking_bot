use poise::serenity_prelude::UserId;

use crate::{db_helper, helper::{self}, Context, Error};

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Check bookings for a given day
#[poise::command(prefix_command, slash_command)]
pub async fn bookings(
    ctx: Context<'_>,
    #[description = "Which day would you like to check?"] day: String,
) -> Result<(), Error> {

    let date_res = helper::process_day(&day);
    if date_res.is_err() {
        let response = format!("Issue with date: \"{}\", please try again", day);
        ctx.say(response).await?;
        return Ok(());
    }
    let date = date_res.unwrap();

    // Lock the Mutex in a block {} so the Mutex isn't locked across an await point
    let bookings_res = {
        // do database stuff
        let db_con = ctx.data().database.lock().unwrap();
        db_helper::booking_funcs::get_bookings(db_con, &date)
    };


    let mut response: String;
    if let Ok(bookings) = bookings_res {
        if bookings.len() == 0 {
            response = format!("There are currently no bookings for {}", date)
        } else {
            response = format!("Bookings for {}\n", date);
            let guild_id = ctx.guild_id().unwrap();
            for (player_1, player_2, reference) in bookings {
                let user_1 = {
                    let x = UserId::new(player_1 as u64).to_user(ctx.http()).await.unwrap();
                    x.nick_in(ctx.http(), guild_id).await.unwrap().to_string()
                };
                let user_2 = {
                    let x = UserId::new(player_2 as u64).to_user(ctx.http()).await.unwrap();
                    x.nick_in(ctx.http(), guild_id).await.unwrap().to_string()
                };

                if let Some(ref_str) = reference {
                    response += &format!("- {} and {} - {}\n", user_1, user_2, ref_str);
                } else {
                    response += &format!("- {} and {}\n", user_1, user_2);
                }
                
            }
        }   
    } else {response = format!("There are currently no bookings for {}", date)};

    ctx.say(response).await?;
    Ok(())
}

/// Book a game on a given day
#[poise::command(prefix_command, slash_command)]
pub async fn book(
    ctx: Context<'_>,
    #[description = "Which day would you like to book for?"] day: String,
    #[description = "The user you want the booking with"] other_user: String,
    #[description = "A reference, e.g. \"crusade\""] reference: Option<String>,
) -> Result<(), Error> {

    let date_res = helper::process_day(&day);
    if date_res.is_err() {
        let response = format!("Issue with date: \"{}\", please try again", day);
        ctx.say(response).await?;
        return Ok(());
    }
    let date = date_res.unwrap();

    {
        let db_con = ctx.data().database.lock().unwrap();
        let _ = db_helper::day_funcs::add_day(db_con, &date);
    }

    let booking_res = {
        let db_con = ctx.data().database.lock().unwrap();
        if let Some(string) = reference {
            db_helper::booking_funcs::book_game(db_con, &date, ctx.author().id.get() as i64, helper::strip_id(&other_user).parse().unwrap(), Some(&string))
        } else {
            db_helper::booking_funcs::book_game(db_con, &date, ctx.author().id.get() as i64, helper::strip_id(&other_user).parse().unwrap(), None)
        }  
    };

    let response: String;
    if let Ok(_) = booking_res {
        response = format!("Game booked for {}", date);
    } else {
        println!("{:?}", booking_res);
        response = "Failed to book game".to_string();
    }
    
    ctx.say(response).await?;
    Ok(())
}


#[poise::command(prefix_command, track_edits, aliases("removeBooking"), slash_command)]
pub async fn remove_booking(
    ctx: Context<'_>,
    #[description = "Which day would you like to remove a booking for?"] day: String,
    #[description = "The user you have the booking with"] other_user: String,
) -> Result<(), Error> {

    let date_res = helper::process_day(&day);
    if date_res.is_err() {
        let response = format!("Issue with date: \"{}\", please try again", day);
        ctx.say(response).await?;
        return Ok(());
    }
    let date = date_res.unwrap();


    let booking_res = {
        let db_con = ctx.data().database.lock().unwrap();
        db_helper::booking_funcs::remove_game(db_con, &date, ctx.author().id.get() as i64, helper::strip_id(&other_user).parse().unwrap())
    };

    let response: String;
    if let Ok(_) = booking_res {
        response = format!("Removed booking on {}", date);
    } else {
        println!("{:?}", booking_res);
        response = "Failed to remove game".to_string();
    }
    
    ctx.say(response).await?;
    Ok(())
}