#[cfg(target_os = "macos")]
static CHROME_PATH: &str = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
#[cfg(not(target_os = "macos"))]
static CHROME_PATH: &str = "/usr/bin/chromium-browser";

pub fn start_browser(port: u16) -> std::io::Result<std::process::Child> {
    std::process::Command::new(CHROME_PATH)
        .arg(format!("--remote-debugging-port={}", port))
        .spawn()
}

// port 3000
pub fn start_server() -> std::io::Result<std::process::Child> {
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--example")
        .arg("server")
        .spawn()
}
