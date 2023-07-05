use dotenv::dotenv;
use flowsnet_platform_sdk::logger;
use openai_flows::{
    chat::{ChatModel, ChatOptions},
    OpenAIFlows,
};
use slack_flows::{listen_to_channel, send_message_to_channel, SlackMessage};
use std::env;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();
    logger::init();
    let workspace: String = match env::var("slack_workspace") {
        Err(_) => "secondstate".to_string(),
        Ok(name) => name,
    };

    let channel: String = match env::var("slack_channel") {
        Err(_) => "collaborative-chat".to_string(),
        Ok(name) => name,
    };

    log::debug!("Workspace is {} and channel is {}", workspace, channel);

    listen_to_channel(&workspace, &channel, |sm| handler(sm, &workspace, &channel)).await;
}

async fn handler(sm: SlackMessage, workspace: &str, channel: &str) {
    let chat_id = workspace.to_string() + channel;
    let co = ChatOptions {
        model: ChatModel::GPT35Turbo,
        restart: false,
        system_prompt: None,
    };
    log::debug!("get OpenAI settings");
    let of = OpenAIFlows::new();
    log::debug!("got text {}", &sm.text);
    if let Ok(c) = of.chat_completion(&chat_id, &sm.text, &co).await {
        log::debug!("got OpenAI response");
        send_message_to_channel(&workspace, &channel, c.choice).await;
        log::debug!("sent to slack");
    }
    log::debug!("done");
}
