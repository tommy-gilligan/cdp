#[cfg(target_os = "macos")]
static CHROME_PATH: &str = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
#[cfg(not(target_os = "macos"))]
static CHROME_PATH: &str = "/usr/bin/chromium-browser";

pub struct Process(std::io::Result<std::process::Child>);

impl Process {
    pub fn browser(port: u16) -> Self {
        Self(
            std::process::Command::new(CHROME_PATH)
                .arg(format!("--remote-debugging-port={}", port))
                .spawn()
        )
    }

    // port 3000
    pub fn server() -> Self {
        Self(
            std::process::Command::new("cargo")
                .arg("run")
                .arg("--example")
                .arg("server")
                .spawn()
        )
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        self.0.as_mut().unwrap().kill().unwrap()
    }
}
