// Tool to automatically rebase branches.

use argh::FromArgs;
use anyhow::Result;

use autorebase::{get_repo_path, autorebase};

#[derive(FromArgs)]
/// Automatically pull the master branch and rebase all branches without
/// upstreams onto it.
struct CliOptions {
    /// the target branch to pull and rebase onto (typically "master" or "develop")
    #[argh(option, default="String::from(\"master\")")]
    onto: String,
}

fn main() -> Result<()> {
    let res = run();
    if res.is_err() {
        // Print a newline because there may be a half finished output
        // (e.g. using `eprint!()` instead of `eprintln!()`.
        eprintln!();
    }
    res
}

fn run() -> Result<()> {
    let options: CliOptions = argh::from_env();

    // Find the repo dir in the same way git does.
    let repo_path = get_repo_path()?;

    autorebase(&repo_path, &options.onto)?;

    Ok(())
}
