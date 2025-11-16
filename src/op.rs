use std::process::Command;

pub struct OnePassword {}

impl OnePassword {
    pub fn get_item(reference: &str) -> String {
        let output = Command::new(if cfg!(target_os = "windows") {
            "op.exe"
        } else {
            "op"
        })
        .arg("read")
        .arg(reference)
        .output()
        .expect("failed to execute process");

        if output.status.success() {
            let token = String::from_utf8(output.stdout).expect("Invalid UTF-8 sequence");
            token.trim().to_owned()
        } else {
            // output the error message from stderr
            let error_message = String::from_utf8(output.stderr).expect("Invalid UTF-8 sequence");
            eprintln!("{}", error_message);
            std::process::exit(1);
        }
    }
}

mod tests {
    #[test]
    fn test_init_1password() {
        use crate::op::OnePassword;
        let token = OnePassword::get_item("op://Private/GitHub/token");
        assert!(!token.is_empty());
    }
}
