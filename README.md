# A general ChatGPT bot for your slack channel

[Deploy this function on flows.network](#deploy-chatgpt-slack-bot-on-your-slack-channel), and you will get a Slack bot that uses ChatGPT to respond to every question in your Slack workspace automatically. Below is an example:

<img width="734" alt="image" src="https://user-images.githubusercontent.com/45785633/224654183-a7ab4dd1-8a85-4304-a68f-c208fbfd50e2.png">

## Deploy your own Slack ChatGPT bot in 3 simple steps

1. Create a bot from a template
2. Authenticate your Slack
3. Add your OpenAI API key

## 0 Prerequisites

1. You will need to bring your own [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

2. You will also need to sign up for [flows.network](https://flows.network/) with your GitHub account. It is free.

## 1 Create a bot from a template

[**Just click here**](https://flows.network/flow/createByTemplate/Slack-Chatgpt)

You can customize the repo name. You will fork this template repo to your GitHub.

Click on the **Create and Build** button.

<img width="943" alt="image" src="https://github.com/flows-network/slack-chatgpt/assets/37167103/40eae6d2-76ed-4fb9-825e-e5286003f4c2">

## 2 Authenticate the Slack integration

 Click the "Connect" button to authenticate your Slack account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on a Slack worksapce. This workspace is the one you entered into the environment variables above.

 * `slack_channel`: Slack channel where you want to deploy the bot. Case sensitive.
 * `slack_workspace`: Slack organization of the Slack channel where you want to deploy the bot. Case sensitive.

## 3 Add your OpenAI API key

Click the "Connect" button to authenticate your OpenAI account. You'll be redirected to a new page where you can copy and paste your OpenAI API key. 

[<img width="450" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">](https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png)

Close the tab and go back to the flow.network page once you are done. Click on **Deploy**.

<img width="937" alt="image" src="https://github.com/flows-network/slack-chatgpt/assets/37167103/f61641f2-897c-43a8-891f-e7340b9d70e6">

## Ready to go

 As soon as the flow function's status becomes `running`, the ChatGPT Slack bot goes live. Go ahead and chat with ChatGPT by sending a message in the channel!
<img width="1404" alt="image" src="https://github.com/flows-network/slack-chatgpt/assets/37167103/1c84d662-b21a-4edb-826f-43ee99ccabbf">

 ## FAQ

### Customize the bot

The bot's source code is available in the GitHub repo you cloned from the template. Feel free to make changes to the source code (e.g., model, context length, API key and prompts) to fit your own needs. If you need help, [ask in Discord](https://discord.gg/ccZn9ZMfFf)!

### Use GPT4

By default, the bot uses GPT3.5. If your OpenAI API key has access to GPT4, you can open the `src/lib.rs` file
in your cloned source code repo, and change `GPT35Turbo` to `GPT4` in the source code. Commit and push the change back to GitHub.
The flows.network platform will automatically detect and rebuild the bot from your updated source code.

### Use the bot on multiple repos

You can [mannually create a new flow](https://flows.network/flow/new) and import the source code repo for the bot (i.e., the repo you cloned from the template).
<img width="879" alt="image" src="https://github.com/flows-network/slack-chatgpt/assets/37167103/8436d1bb-b47e-4b4b-bb9c-ea38dc8ff5fc">
Then, you can use the flow config to specify the `slack_channel` and `slack_workspace` to point to the target slack channel you need to deploy the bot in. Deploy and authorize access.

You can repeat this for all target slack channels you would like to deploy this bot in.

> You can have a single flow function repo deployed as the source code for multiple bots. When you update the source code in the repo, and push it to GitHub, it will change the behavior of all the bots.

> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!


