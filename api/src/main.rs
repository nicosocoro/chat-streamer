use anyhow::Result;
use gstreamer as gst;

fn main() -> Result<()> {
    env_logger::init();
    gst::init()?;

    println!("GStreamer version: {}", gst::version_string());

    Ok(())
}