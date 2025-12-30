use anyhow::{Error, Result};
use gstreamer::{self as gst, glib::GString};

pub fn gstreamer_version() -> Result<GString, Error> {
    return Ok(gst::version_string());
}