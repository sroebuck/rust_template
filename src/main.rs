#[macro_use]
extern crate log;
extern crate env_logger;

fn main() -> Result<(), Box<std::error::Error>> {
    env_logger::init();

    info!("Hello, world!");

    Ok(())
}
