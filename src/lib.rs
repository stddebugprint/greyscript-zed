use zed_extension_api::{self as zed, Command, Extension, LanguageServerId, Result, Worktree};

struct GreyscriptExtension;

impl Extension for GreyscriptExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let command = worktree
            .which("greybel-languageserver")
            .ok_or_else(|| "greybel-languageserver not found in PATH".to_string())?;

        Ok(Command {
            command,
            args: vec!["--stdio".to_string()],
            env: vec![],
        })
    }
}

zed::register_extension!(GreyscriptExtension);
