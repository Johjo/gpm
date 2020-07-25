use git2::{Repository, DescribeOptions};

fn repository_version() -> Result<String, git2::Error> {
    let mut options = DescribeOptions::new();

    options.describe_tags().show_commit_oid_as_fallback(true);
    
    let repo = Repository::discover(env!("CARGO_MANIFEST_DIR"))?;
    let descr = repo.describe(&options)?;
    
    Ok(descr.format(None)?)
}

fn main() {
    println!("cargo:rustc-env=GIT_DESCRIBE={}", repository_version().unwrap());
}
