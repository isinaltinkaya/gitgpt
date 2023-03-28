# gitgpt: Make ChatGPT write commit messages for you

This program is a Rust implementation of a bot designed to help people write concise git commit messages. The bot reads the git change summary and fills the commit message template based on the changes.

## Installation
To use this program, first clone the repository and build the program using Cargo.

```bash
git clone https://github.com/isinaltinkaya/gitgpt.git
cd gitgpt
cargo build --release
```

## Usage
To use the bot, you will need to set the `OPENAI_API_KEY` environment variable to your OpenAI API key.

```bash
export OPENAI_API_KEY=your_api_key_here
```

Then, simply run the gitgpt binary inside the directory of the git repository you want to write commit message for.


```sh
./target/release/gitgpt
```



## How It Works
This program uses the ChatGPT API from OpenAI to generate the git commit message template based on the changes in the git repository. It first gets the output of git status and git diff commands to determine the changes made in the repository. Then, it generates a prompt with the git commit message template and the list of changed files. The program uses the user's OpenAI API key to send the prompt to the ChatGPT API and receives the generated git commit message template as a response. Finally, the program writes the git commit message template to a file and asks the user if they want to open the file for editing. If the user chooses to open the file, the program executes `git commit -eF git_message.txt` to open the file for editing.
