use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    new_client, DiscordIpc,
};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn rpc() {
    let mut client = new_client("757942105363447878").unwrap();
    loop {
        if client.connect().is_ok() {
            break;
        }
    }

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    loop {
        let payload = Activity::new()
            .state("sage.exe")
            .details("An MCPE Client")
            .assets(
                Assets::new()
                    .large_image("index_2_")
                    .small_image("diamond_sword"),
            )
            .buttons(vec![Button::new(
                "Repository",
                "https://github.com/Cqdet/sage",
            )])
            .timestamps(Timestamps::new().start(time));

        if client.set_activity(payload).is_err() && client.reconnect().is_ok() {
            continue;
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
