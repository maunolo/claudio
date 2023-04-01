use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};

use rusty_vban::utils::cpal::Device;

pub fn list_devices() -> Result<()> {
    let host = cpal::default_host();
    let devices = host.devices()?;
    for device in devices {
        let device_type = if device.is_output() {
            "Output"
        } else if device.is_input() {
            "Input"
        } else {
            "Unknown"
        };
        println!("{}: {}", device_type, device.name()?);
    }

    Ok(())
}
