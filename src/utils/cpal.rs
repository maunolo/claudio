use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait};

pub trait Host {
    fn find_input_device(&self, name: &str) -> Option<cpal::Device>;
    fn find_output_device(&self, name: &str) -> Option<cpal::Device>;
}

impl Host for cpal::Host {
    fn find_input_device(&self, name: &str) -> Option<cpal::Device> {
        match name {
            "default" => self.default_input_device(),
            _ => {
                if let Ok(mut devices) = self.devices() {
                    devices.find(|d| {
                        if let Ok(d_name) = d.name() {
                            d_name == name
                        } else {
                            false
                        }
                    })
                } else {
                    None
                }
            }
        }
    }

    fn find_output_device(&self, name: &str) -> Option<cpal::Device> {
        match name {
            "default" => self.default_output_device(),
            _ => {
                if let Ok(mut devices) = self.devices() {
                    devices.find(|d| {
                        if let Ok(d_name) = d.name() {
                            d_name == name
                        } else {
                            false
                        }
                    })
                } else {
                    None
                }
            }
        }
    }
}

pub trait Device {
    fn default_config(&self) -> Result<cpal::SupportedStreamConfig>;

    fn is_output(&self) -> bool;

    fn is_input(&self) -> bool;

    fn is_default_output(&self, host: &cpal::Host) -> bool;

    fn is_default_input(&self, host: &cpal::Host) -> bool;

    fn compare_name(&self, device: &cpal::Device) -> bool;
}

impl Device for cpal::Device {
    fn default_config(&self) -> Result<cpal::SupportedStreamConfig> {
        if let Ok(config) = self.default_output_config() {
            return Ok(config);
        }
        if let Ok(config) = self.default_input_config() {
            return Ok(config);
        }

        Err(anyhow!("No default config found"))
    }

    fn is_output(&self) -> bool {
        if let Ok(_) = self.default_output_config() {
            return true;
        }
        false
    }

    fn is_input(&self) -> bool {
        if let Ok(_) = self.default_input_config() {
            return true;
        }
        false
    }

    fn is_default_output(&self, host: &cpal::Host) -> bool {
        if let Some(default_output) = host.default_output_device() {
            return default_output.compare_name(&self);
        }
        false
    }

    fn is_default_input(&self, host: &cpal::Host) -> bool {
        if let Some(default_input) = host.default_input_device() {
            return default_input.compare_name(&self);
        }
        false
    }

    fn compare_name(&self, device: &cpal::Device) -> bool {
        if let Ok(name) = self.name() {
            if let Ok(device_name) = device.name() {
                return name == device_name;
            }
        }
        false
    }
}

pub trait Devices {
    fn find_input(&mut self, name: &str) -> Option<cpal::Device>;

    fn find_output(&mut self, name: &str) -> Option<cpal::Device>;
}

impl Devices for cpal::Devices {
    fn find_input(&mut self, name: &str) -> Option<cpal::Device> {
        self.filter(|d| d.is_input()).find(|d| {
            if let Ok(d_name) = d.name() {
                d_name == name
            } else {
                false
            }
        })
    }

    fn find_output(&mut self, name: &str) -> Option<cpal::Device> {
        self.filter(|d| d.is_output()).find(|d| {
            if let Ok(d_name) = d.name() {
                d_name == name
            } else {
                false
            }
        })
    }
}
