use crate::{helper::{self, strip_id}, Context, Error};

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
        let response = format!("Issue with date format: {}, please try again", day);
        ctx.say(response).await?;
        return Ok(());
    }
    let date = date_res.unwrap();

    // Lock the Mutex in a block {} so the Mutex isn't locked across an await point
    {
        // do database stuff
        let mut _db_con = ctx.data().database.lock().unwrap();

    };

    let response = format!("Checking bookings for {}", date);
    ctx.say(response).await?;
    Ok(())
}

/// Book a game on a given day
#[poise::command(prefix_command, slash_command)]
pub async fn book(
    ctx: Context<'_>,
    #[description = "Which day would you like to book for?"] day: String,
    #[description = "The user you want the booking with"] other_user: String,
) -> Result<(), Error> {
    
    println!("This user: {}", ctx.author().id);
    println!("Other user: {}", strip_id(&other_user));
    let response = format!("Booking for a day with {}", other_user);
    ctx.say(response).await?;
    Ok(())
}


#[poise::command(prefix_command, track_edits, aliases("removeBooking"), slash_command)]
pub async fn remove_booking(
    ctx: Context<'_>,
    #[description = "Which day would you like to remove a booking for?"] day: String,
    #[description = "The user you have the booking with"] other_user: String,
) -> Result<(), Error> {

    let response = format!("Removing booking for a day with {}", other_user);
    ctx.say(response).await?;
    Ok(())
}