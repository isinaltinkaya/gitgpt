use chatgpt::prelude::*;
use chatgpt::types::CompletionResponse;
use std::{process::{Command,Stdio}, fs::File, io::{self, Write}};



#[tokio::main]
async fn main() -> Result<()> {

    // get the API key from an environment variable OPENAI_API_KEY
    let key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");

    // create new ChatGPT client
    let gpt = ChatGPT::new(key)?;

    let cwd = std::env::current_dir().unwrap();

    // get the output of `git status`
    let status_output = Command::new("git")
        .current_dir(&cwd)
        .args(&["status", "--porcelain"])
        .output()
        .expect("Failed to execute git status command");

    // get the output of `git diff`
    let diff_output= Command::new("git")
        .current_dir(&cwd)
        .args(&["diff", "--unified=0", "--cached"])
        .output()
        .expect("Failed to execute git diff command");

    let status_str = String::from_utf8_lossy(&status_output.stdout).to_string();
    let diff_str = String::from_utf8_lossy(&diff_output.stdout).to_string();

    let mut changed_files = Vec::new();

    for line in diff_str.lines() {
        let mut line_str = line.trim_start_matches("+++ b/+").to_string();
        let _=line_str.remove(0);

        if !changed_files.contains(&line_str) {
            changed_files.push(line_str);
        }
    }

    for line in status_str.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            let file_path = parts[1];

            if !changed_files.contains(&file_path.to_string()) {
                changed_files.push(file_path.to_string());
            }
        }
    }

    let prompt = format!(
        "You are a bot designed for helping people write concise git commit messages. You read the git change summary and fill the commit message template based on the changes.

        Following is the commit message template you use. You fill the empty lines between # characters, except for the empty line between title and body. You do not change anything in the lines starting with #. Maximum title length is 50 characters. You summarize all the changes in one git commit message.

# Title:
[type][optional scope] Title
#################################################
# Title length: 50 chars - - - - - - - - - - -> #
#################################################
#
#
#
# The line below is the empty line between title and body

#
#
#
#
#
# Body: Explain *what* and *why* (not *how*) 
# Body length: Wrap at 72 chars  - - - - - - - - - - - - - - - - - -> #

#


Following are the commit types to use in commit titles:
[type]:
[feat] - A new feature
[fix] - A bug fix
[perf] - A code change that improves performance
[test] - Code unit testing related changes
[build] - Build related changes
[chore] - Build process or auxiliary tool changes
[doc] - Documentation-related changes
[style] - Markup, formatting, typos etc


Following is an example commit summary:

diff --git a/ngsAMOVA.cpp b/ngsAMOVA.cpp
index 53c371f..e88851a 100755
--- a/ngsAMOVA.cpp
ngsAMOVA.cpp
@@ -481,0 +482,3 @@ int main(int argc, char **argv)
+// perform neighbor joining based on the dxy satistics
+void neighborJoiningDxy();
+

Following is an example commit message template you write:


# Title:
[feat][method] Add new method neighbor joining
#################################################
#################################################
# Title length: 50 chars - - - - - - - - - - -> #
#################################################
#
#
#
# The line below is the empty line between title and body

based on the dxy 
#
#
#
#
#
# Body: Explain *what* and *why* (not *how*) 
# Body length: Wrap at 72 chars  - - - - - - - - - - - - - - - - - -> #
Add method neighborJoiningDxy for perform neigbor joining
#




        Following is a git change summary: Updated the following files:\n{}\nFill the template based on the git change summary:
    ",

        changed_files.join("\n")
    );

    println!("{}",prompt);


    let response: CompletionResponse = gpt 
        .send_message(&prompt)
        .await?;

    let mut file = File::create("git_message.txt").unwrap();
    file.write_all(response.message().content.to_string().as_bytes()).unwrap();

    println!("{}", response.message().content);
    
    // ask the user if they want to open the file for editing
    let should_edit = ask_yes_no_question("Do you want to open git_message.txt for editing?");

    if should_edit {
        // open the file for editing with git commit -eF
        Command::new("git")
            .current_dir(&cwd)
            .args(&["commit", "-eF", "git_message.txt"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap();
    }


    Ok(())
}



fn ask_yes_no_question(question: &str) -> bool {
    loop {
        print!("{} [y/n]: ", question);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.to_lowercase().trim() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Invalid input. Please enter 'y' or 'n'.");
                continue;
            }
        }
    }
}
