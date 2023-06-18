use git2::Repository;

fn convert_to_forge_url(url: &str) -> String {
    let mut url = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("git@"))
        .unwrap_or(url)
        .to_owned();

    url = url.strip_suffix(".git").unwrap_or(&url).to_owned();
    url = url.replace(':', "/");

    format!("https://{}", url)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::discover(".")?;
    let remotes = repo.remotes()?;

    if remotes.is_empty() {
        println!("Repository has no remotes");
    } else {
        for name in remotes.iter().flatten() {
            let remote = repo.find_remote(name)?;
            let remote_url = remote.url().ok_or("remote has no url")?;
            let forge_url = convert_to_forge_url(remote_url);
            println!("{}", forge_url);
            webbrowser::open(&forge_url)?;
        }
    }
    Ok(())
}
