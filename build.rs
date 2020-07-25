use git2::{Repository, DescribeOptions};

fn repository_version(repo: Repository) -> Result<String, git2::Error> {
    let mut options = DescribeOptions::new();

    options.describe_tags().show_commit_oid_as_fallback(true);
    
    let descr = repo.describe(&options)?;
    
    Ok(descr.format(None)?)
}

fn main() {
    let repo = Repository::discover(env!("CARGO_MANIFEST_DIR")).unwrap();
    let head_ref_path = repo.head().unwrap().name().unwrap().to_owned();
    let version = repository_version(repo).unwrap();

    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/{}", head_ref_path);
    println!("cargo:rustc-env=GIT_DESCRIBE={}", version);
}
