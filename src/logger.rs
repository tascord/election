use log::{LevelFilter, info, warn};
use std::{
    collections::HashMap,
    env::{self, args},
    fs::{self, File, OpenOptions},
    io::{Write, stdout},
};
struct Log(File);
impl Write for Log {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        stdout().write(buf).and_then(|_| self.0.write(buf))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        stdout().flush().and_then(|_| self.0.flush())
    }
}

impl Log {
    pub fn new(path: String) -> Box {
        Self(OpenOptions::new().create(true).append(true).open("elc.log"))
    }
}

fn logger() {
    let mut builder = pretty_env_logger::formatted_builder();
    builder.target(env_logger::fmt::Target::Pipe(Log(OpenOptions::new()
        .create(true)
        .append(true)
        .open("elc.log"))));

    builder.filter_level(if env::var("RUST_LOG").is_ok() {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });
    builder.init();
}
