use slack_flows::{listen_to_channel, send_message_to_channel};
use openai_flows::{CompletionRequest, create_completion};
use store_flows::{get, set};
use serde_json::json;

#[no_mangle]
pub fn run() {
    listen_to_channel("secondstate", "collaborative-chat", |sm| {
        let prompt = get("prompt").unwrap_or_default();
        let prompt = prompt.as_str().unwrap_or_default();
        if prompt.len() == 0 {
            send_message_to_channel("secondstate", "collaborative-chat", "Starting a new conversation since the last interaction was over 5 minutes ago".to_owned());
        }

        let mut prompt = prompt.to_owned() + " " + &sm.text;
        if prompt.len() > 2047 {
            let (_discard, keep) = prompt.split_at(prompt.len() - 2047);
            prompt = keep.to_owned();
        }
        set("prompt", json!(prompt));
        let cr = CompletionRequest {
            prompt: prompt,
            n: 1,
            max_tokens: 2048,
            ..Default::default()
        };

        let r = create_completion("Agent", cr);
        r.iter().for_each(|c| {
            send_message_to_channel("secondstate", "collaborative-chat", c.to_string());
        });
    });
}
