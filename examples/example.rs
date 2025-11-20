use discord_presence_rs::activities::{
    Activity, ActivityType, Assets, Button, Party, StatusDisplayType, Timestamps,
};
use discord_presence_rs::discord_connection::Client;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let client_id = "YOUR_CLIENT_ID";

    let mut client = match Client::new(client_id) {
        Ok(client) => {
            println!("Connected!");
            client
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let activity = Activity::new()
        .set_details("Discord Presence".to_string())
        .set_state("Kaizenium Foundation".to_string())
        .set_activity_type(ActivityType::Playing)
        .set_assets(
            Assets::new()
                .set_large_image("image1".to_string())
                .set_large_text("image1".to_string())
                .set_large_url("https://www.youtube.com/@KaizeniumFoundation".to_string())
                .set_small_image("image2".to_string())
                .set_small_text("image2".to_string())
                .set_small_url("https://github.com/mertbolukbasi".to_string()),
        )
        .set_party(Party::new().set_id("party".to_string()).set_size(1, 10))
        .set_buttons(vec![
            Button::new()
                .set_label("Youtube".to_string())
                .set_url("https://www.youtube.com/@KaizeniumFoundation".to_string()),
            Button::new()
                .set_label("GitHub".to_string())
                .set_url("https://github.com/mertbolukbasi".to_string()),
        ])
        .set_status_display_type(StatusDisplayType::Details)
        .set_timestamps(Timestamps::new().set_start(start_time));

    if let Err(e) = client.set_activity(activity) {
        eprintln!("Error: {}", e);
    } else {
        println!("Success!");
    }

    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
