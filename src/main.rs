use git2::Repository;

fn main() {
    let repo = Repository::discover(".").unwrap();
    let remotes = repo.remotes().unwrap();

    if remotes.is_empty() {
        println!("Repository has no remotes");
    } else {
        for name in remotes.iter() {
            let remote = repo.find_remote(name.unwrap()).unwrap();
            let url = remote.url().unwrap();

            let url = url.strip_prefix("https://").unwrap_or(url);
            let url = url.strip_prefix("git@").unwrap_or(url);
            let url = url.strip_suffix(".git").unwrap_or(url);
            let url = url.replace(':', "/");
            let url = format!("https://{}", url);

            webbrowser::open(&url).unwrap();
        }
    }
}
