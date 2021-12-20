use anyhow::Result;
use colored::*;
use git_commands::*;

mod trim;
use trim::*;

/// Autosquash automatically squashes onto the merge base.
pub fn autosquash(
    onto_branch: &str,
) -> Result<()> {

    eprint!("{}", "• Getting merge base...".yellow());
    let merge_base = get_merge_base(onto_branch)?;
    eprintln!("\r{}", "• Getting merge base...".green());

    eprint!("{}", "• Getting commit messages...".yellow());
    let message = get_commit_messages(&merge_base, "HEAD")?;
    eprintln!("\r{}", "• Getting commit messages...".green());

    eprint!("{}", "• Resetting to merge base...".yellow());
    reset_to(&merge_base)?;
    eprintln!("\r{}", "• Resetting to merge base...".green());

    eprint!("{}", "• Committing...".yellow());
    make_commit(&message)?;
    eprintln!("\r{}", "• Committing...".green());

    Ok(())
}


fn get_merge_base(branch: &str) -> Result<String> {
    let commit = git_cwd(&["merge-base", "HEAD", &branch])?.stdout;
    let commit = std::str::from_utf8(commit.trim_ascii_whitespace())?;
    Ok(commit.to_owned())
}

fn get_commit_messages(from: &str, to: &str) -> Result<String> {
    let messages = git_cwd(&["--no-pager", "log", "--format=%B", &format!("{}..{}", from, to)])?.stdout;
    let messages = std::str::from_utf8(&messages)?;
    Ok(messages.to_owned())
}

fn reset_to(commit: &str) -> Result<()> {
    git_cwd(&["reset", commit])?;
    Ok(())
}

fn make_commit(message: &str) -> Result<()> {
    git_cwd(&["commit", "-m", message])?;
    Ok(())
}
