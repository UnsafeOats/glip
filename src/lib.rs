#![allow(dead_code)]
use std::process::{Command, Stdio};
use std::env;
use anyhow::{bail, Result, Context};

pub enum Os {
    Windows,
    Linux,
    Mac,
    Unsupported,
}

impl Os {
    pub fn get() -> Self {
        match env::consts::OS {
            "windows" => Os::Windows,
            "linux" => Os::Linux,
            "macos" => Os::Mac,
            _ => Os::Unsupported,
        }
    }
}

struct GlobalClip;

impl GlobalClip {
    pub fn set(val: &str) -> Result<()> {
        let echo_val = Command::new("echo")
            .arg(val)
            .stdout(Stdio::piped())
            .spawn()?;
        match Os::get() {
            Os::Windows => {
                Command::new("clip")
                    .stdin(echo_val.stdout.context("[error] Failed to capture input string to copy.")?)
                    .spawn()?;
            }
            Os::Linux => {
                Command::new("xclip")
                    .arg("-selection")
                    .arg("clipboard")
                    .stdin(echo_val.stdout.context("[error] Failed to capture input string to copy.")?)
                    .spawn()?;
            }
            Os::Mac => {
                Command::new("pbcopy")
                    .stdin(echo_val.stdout.context("[error] Failed to capture input string to copy.")?)
                    .spawn()?;
            }
            Os::Unsupported => bail!("[error] Unsupported OS"),
        }
        Ok(())
    }

    pub fn get() -> Result<String> {
        let output = match Os::get() {
            Os::Windows => {
                Command::new("powershell")
                    .arg("Get-Clipboard")
                    .output()?
            }
            Os::Linux => {
                Command::new("xclip")
                    .arg("-selection")
                    .arg("clipboard")
                    .arg("-o")
                    .output()?
            }
            Os::Mac => {
                Command::new("pbpaste")
                    .output()?
            }
            Os::Unsupported => bail!("[error] Unsupported OS"),
        };
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let val = "test val";
        GlobalClip::set(val).unwrap();
        let result = GlobalClip::get().unwrap();
        assert_eq!(result, val);
    }
}
