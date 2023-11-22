use ::std::fs;
use ::std::process::{Command, Output};
use dialoguer::{theme::ColorfulTheme, Confirm};

fn main() {
    // 1.
    let status_output = check_git_status();

    // 2.
    let git_diff = get_git_diff();

    // 3.
    let api_key: () = read_api_key();

    // 4.
    let commit_message = fetch_ai_commit_message(&api_key, &git_diff);
}

fn check_git_status() {}

fn get_git_diff() {}

fn read_api_key() -> Result<String, Box<dyn std::error::Error>> {}

fn fetch_ai_commit_message(key: &str, diff: &str) {}
