use chrono::{DateTime, TimeZone, Utc};
use git2::Repository;
use std::error::Error;

fn main() {
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");

    let (git_hash, commit_date) =
        get_git_info().unwrap_or_else(|_| (String::from("unknown"), String::from("unknown")));

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=COMMIT_DATE={}", commit_date);
}

fn get_git_info() -> Result<(String, String), Box<dyn Error>> {
    let repo = Repository::open_from_env().or_else(|_| Repository::discover("."))?;

    let head = repo.head()?;
    let commit = head.peel_to_commit()?;

    let hash = commit.id().to_string();

    let timestamp = commit.time();
    let datetime: DateTime<Utc> = Utc
        .timestamp_opt(timestamp.seconds(), 0)
        .single()
        .ok_or("Invalid timestamp")?;
    let date = datetime.format("%Y-%m-%d").to_string();

    Ok((hash, date))
}
