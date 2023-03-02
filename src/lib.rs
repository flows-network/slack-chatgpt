use slack_flows::{listen_to_channel, send_message_to_channel};
use openai_flows::{chat_completion};

#[no_mangle]
pub fn run() {
    listen_to_channel("secondstate", "collaborative-chat", |sm| {
        let chat_id = "secondstate-collaborative_chat";
        let c = chat_completion("Agent", chat_id, &sm.text);
        if let Some(c) = c {
            if c.new_conversation {
                send_message_to_channel("secondstate", "collaborative-chat", "Let's start a new conversation!".to_string());
            }
            send_message_to_channel("secondstate", "collaborative-chat", c.choice);
        }
    });
}
