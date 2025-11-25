use std::{io::ErrorKind, process::Command};

pub struct OnePassword {}

impl OnePassword {
    pub fn get_item(reference: &str) -> String {
        let exe = if cfg!(target_os = "windows") {
            "op.exe"
        } else {
            "op"
        };
        let output = Command::new(exe)
            .arg("read")
            .arg(reference)
            .arg("-n")
            .output()
            .unwrap_or_else(|e| {
                if e.kind() == ErrorKind::NotFound {
                    eprintln!("Error: 'op' command not found. Please ensure that 1Password CLI is installed and accessible in your PATH.");
                } else {
                    eprintln!("Failed to execute 'op' command: {}", e);
                }
                std::process::exit(1);
            });

        if output.status.success() {
            let token =
                String::from_utf8(output.stdout).expect("Invalid UTF-8 output from op command");
            token
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            eprintln!("{}", error_message);
            std::process::exit(1);
        }
    }
}

mod tests {
    #[test]
    fn test_init_1password() {
        use crate::op::OnePassword;
        let token = OnePassword::get_item("op://Private/GitHub/token".into());
        assert!(!token.is_empty());
        assert!(token.len() > 10);
        assert!(!token.contains('\n'));
    }
}
