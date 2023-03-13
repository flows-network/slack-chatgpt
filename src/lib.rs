use slack_flows::{listen_to_channel, send_message_to_channel};
use openai_flows::{chat_completion, ChatOptions};
use std::env;
use dotenv::dotenv;

#[no_mangle]
pub fn run() {
    dotenv().ok();
    let workspace: String = match env::var("workspace") {
        Err(_) => "secondstate".to_string(),
        Ok(name) => name,
    };

    let channel: String = match env::var("channel") {
        Err(_) => "collaborative-chat".to_string(),
        Ok(name) => name,
    };
    let openai_key_name: String = match env::var("openai_key_name") {
        Err(_) => "chatmichael".to_string(),
        Ok(name) => name,
    };

    listen_to_channel(&workspace, &channel, |sm| {
        let chat_id = workspace.clone() + &channel;
        let c = chat_completion(&openai_key_name, &chat_id, &sm.text, &ChatOptions::default());
        if let Some(c) = c {
            if c.restarted {
                send_message_to_channel(&workspace, &channel, "Let's start a new conversation!".to_string());
            }
            send_message_to_channel(&workspace, &channel, c.choice);
        }
    });
}
