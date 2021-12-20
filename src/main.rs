// Tool to automatically rebase branches.

use anyhow::Result;
use argh::FromArgs;

use autosquash::autosquash;

#[derive(FromArgs)]
/// Automatically squash commits back to the merge base with master.
struct CliOptions {
    /// the target branch to find the merge base with (typically "master" or "develop")
    #[argh(option, default = "String::from(\"master\")")]
    onto: String,

    /// RUST_LOG-style logging string, e.g. --log debug
    #[argh(option)]
    log: Option<String>,
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

    env_logger::Builder::new().parse_filters(&options.log.unwrap_or_default()).init();

    autosquash(&options.onto)?;

    Ok(())
}
