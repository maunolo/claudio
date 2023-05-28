use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};

use rusty_vban::utils;
use rusty_vban::utils::cpal::Device;

pub fn list_devices(host_name: Option<&str>) -> Result<()> {
    let host = utils::cpal::host_by_name(host_name.unwrap_or("default"))?;
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

pub fn default_output(host_name: Option<&str>) -> Result<()> {
    let host = utils::cpal::host_by_name(host_name.unwrap_or("default"))?;
    let device = host.default_output_device().unwrap();
    println!("Default output device: {}", device.name().unwrap());

    Ok(())
}

pub fn default_input(host_name: Option<&str>) -> Result<()> {
    let host = utils::cpal::host_by_name(host_name.unwrap_or("default"))?;
    let device = host.default_input_device().unwrap();
    println!("Default input device: {}", device.name().unwrap());

    Ok(())
}

pub fn list_backends() -> Result<()> {
    let hosts = cpal::available_hosts();
    for host in hosts {
        println!("Backend: {}", host.name());
    }

    Ok(())
}
