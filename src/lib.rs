use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree};

struct VixenExtension;

impl zed::Extension for VixenExtension {
    fn new() -> Self {
        VixenExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let path = worktree.which("vx").ok_or_else(|| {
            "vx not found in PATH. Please install vixen and ensure vx is on your PATH.".to_string()
        })?;

        Ok(Command {
            command: path,
            args: vec!["lsp".to_string()],
            env: vec![],
        })
    }
}

zed::register_extension!(VixenExtension);
