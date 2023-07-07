# A General ChatGPT Bot for Your Slack Channel(s)

[Deploy this function on flows.network](#deploy-chatgpt-slack-bot-on-your-slack-channel), and you will get a Slack bot that uses ChatGPT to respond to every question in your Slack workspace automatically. Below is an example:

<img width="734" alt="image" src="https://user-images.githubusercontent.com/45785633/224654183-a7ab4dd1-8a85-4304-a68f-c208fbfd50e2.png">

## Deploy your own Slack ChatGPT bot in 3 simple steps

1. Create a bot from a template
2. Authenticate your Slack
3. Add your OpenAI API key

### 0 Prerequisites

1. You will need to bring your own [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

2. You will also need to sign up for [flows.network](https://flows.network/) with your GitHub account. It is free.

### 1 Create a bot from a template

[**Just click here**](https://flows.network/flow/createByTemplate/Slack-Chatgpt)

You can customize the repo name. You will fork this template repo to your GitHub.

Click on the **Create and Build** button.

<img width="943" alt="image" src="https://github.com/flows-network/slack-chatgpt/assets/37167103/40eae6d2-76ed-4fb9-825e-e5286003f4c2">

### 2 Authenticate the bot to access Slack

Click the "+ Authenticate" button to authorize your Slack account. 
<img width="1409" alt="image" src="https://github.com/flows-network/slack-chatgpt/assets/37167103/60c944c1-ccaa-4582-8727-5d5d1bde7958">

You'll enter which Slack channel you want your bot to be deployed in.

 * `slack_channel`: Slack channel where you want to deploy the bot. Case sensitive.
 * `slack_workspace`: Slack organization of the Slack channel where you want to deploy the bot. Case sensitive.

Enter your Slack workspace and channel respectively in the red boxes below.
![image](https://github.com/flows-network/github-star-slack-messenger/assets/45785633/0d9ac244-f327-4366-972c-47ef05472057)


Next, let's grant flows.network to access the Slack channel you just entered. The Slack channel must be public.

Click the "Connect/+ Add new authentication" button to authenticate your Slack account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network` bot on your workspace. This workspace is the one you entered into the `slack_workspace` above.

### 3 Add your OpenAI API key

Click the "Connect" button to authenticate your OpenAI account. You'll be redirected to a new page where you can copy and paste your OpenAI API key. 

[<img width="450" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">](https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png)

Close the tab and go back to the flow.network page once you are done. Click **Deploy**.

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

### Deploy the bot on different Slack channels

You can [manually create a new flow](https://flows.network/flow/new) and import the source code repo for the bot (i.e., the repo you cloned from the template). 

<img width="1125" alt="image" src="https://github.com/flows-network/slack-chatgpt/assets/37167103/0be8666b-cfb6-4921-bb63-883b9a99e289">
Choose a template repo and enter the `slack_channel` and `slack_workspace` to point to the target slack channel you need to deploy the bot in.
<img width="1138" alt="image" src="https://github.com/flows-network/slack-chatgpt/assets/37167103/cb9aadbb-5849-4889-bcc5-1f8dc8376a37">

Click **Build**.

<img width="1069" alt="image" src="https://github.com/flows-network/slack-chatgpt/assets/37167103/b8c32f02-5ead-4344-95fc-a3d09f0da0b1">

You have already authorized the access to your Slack. Click **Continue**.
<img width="1146" alt="image" src="https://github.com/flows-network/slack-chatgpt/assets/37167103/5c713980-fe60-4e9b-be2b-ca96977c2d6b">

You have already entered your OpenAI key before. click **Deploy**.

You can repeat this for all target slack channels you would like to deploy this bot in.

> You can have a single flow function repo deployed as the source code for multiple bots. When you update the source code in the repo, and push it to GitHub, it will change the behavior of all the bots.

> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!


