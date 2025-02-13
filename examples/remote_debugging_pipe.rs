#![feature(cfg_version)]
#![feature(anonymous_pipe)]
#![feature(c_str_module)]

use cdp::DomainClients;
use nix::unistd::{ForkResult, fork};
use std::{fs::File, os::fd::AsRawFd};
use tokio::runtime::Runtime;

fn main() {
    // placeholder 3 and 4
    let f = File::open("/dev/null").unwrap();
    let g = File::open("/dev/null").unwrap();
    assert_eq!(f.as_raw_fd(), 3);
    assert_eq!(g.as_raw_fd(), 4);

    #[cfg(not(version("1.86")))]
    let (reader_a, writer_a) = std::pipe::pipe().unwrap();
    #[cfg(not(version("1.86")))]
    let (reader_b, writer_b) = std::pipe::pipe().unwrap();
    #[cfg(version("1.86"))]
    let (reader_a, writer_a) = std::io::pipe().unwrap();
    #[cfg(version("1.86"))]
    let (reader_b, writer_b) = std::io::pipe().unwrap();

    nix::unistd::dup2(reader_a.as_raw_fd(), 3).unwrap();
    nix::unistd::dup2(writer_b.as_raw_fd(), 4).unwrap();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let mut client = cdp::PipeClient::new(writer_a, reader_b);
                let response = client
                    .target()
                    .create_target(
                        "http://arstechnica.com".to_owned(),
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .await;
                let response = client
                    .target()
                    .attach_to_target(response.unwrap().target_id, Some(true))
                    .await
                    .unwrap();
                client.set_session_id(response.session_id);
                client.page().enable().await.unwrap();
                client.network().enable(Some(65535)).await.unwrap();
                client.network().set_cache_disabled(true).await.unwrap();
            });
        }
        Ok(ForkResult::Child) => {
            let h = File::open("/dev/null").unwrap();
            let i = File::open("/dev/null").unwrap();

            nix::unistd::dup2(h.as_raw_fd(), 1).unwrap();
            nix::unistd::dup2(i.as_raw_fd(), 2).unwrap();

            let _ = nix::unistd::execv(
                c"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
                &[c"chrome", c"--remote-debugging-pipe"],
            );
        }
        Err(_) => println!("Fork failed"),
    }
}
