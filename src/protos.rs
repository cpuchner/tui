use inquire::MultiSelect;

use crate::dev::read_repos;

pub fn bump_protos() -> Result<(), String> {
    let socket_path =
        std::env::var("KITTY_SOCKET").unwrap_or_else(|_| super::DEFAULT_SOCKET_PATH.to_string());

    let repos = read_repos();

    let selection = MultiSelect::new("Choose repos:", repos)
        .prompt()
        .expect("a selction");

    let kitty_state = super::kitty::find_or_start_kitty(socket_path.as_str())?;

    // TODO: make work for golang
    for repo in selection {
        let cmd = format!(
            "cd {} && git stash --include-untracked && git checkout main && git pull && yarn add eucalyptusvc/protobufs@latest",
            repo.path,
        );

        let tab = &kitty_state.tabs.iter().find(|t| t.title == repo.name);
        if tab.is_none() {
            super::kitty::launch_tab(&socket_path, &repo.name)?;
        } else {
            super::kitty::focus_tab(&socket_path, &repo.name)?;
        }

        super::kitty::send_text(&socket_path, &cmd)?;
    }

    Ok(())
}
