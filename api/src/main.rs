use anyhow::Result;

mod utils;

fn main() -> Result<()> {
    let version = utils::version::gstreamer_version()?;
    println!("GStreamer version: {}", version);
    return Ok(());
}
