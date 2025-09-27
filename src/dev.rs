pub struct Repo {
    pub name: String,
    pub path: String,
}

impl std::fmt::Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

pub fn read_repos() -> Vec<Repo> {
    let home = std::env::var("HOME").expect("HOME environment variable not set");
    let dev_dir = format!("{}/dev/euc", home);

    std::fs::read_dir(&dev_dir)
        .expect("reading directory")
        .map(|e| {
            let name = e
                .expect("entry to be valid")
                .file_name()
                .to_string_lossy()
                .into_owned();

            Repo {
                path: format!("{}/{}", &dev_dir, &name),
                name,
            }
        })
        .collect()
}
