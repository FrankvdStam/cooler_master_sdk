pub mod ffi;
use ffi::*;

pub struct CoolerMasterDevice
{
    device_index: DeviceIndex,
    pub color_matrix: ColorMatrix,
    pub layout: KeyboardLayout,
    pub sdk_version: i32,
    has_led_control: bool,
}

impl CoolerMasterDevice
{
    ///Constructs a struct that acts as a handle to it's available functions.
    ///Claims control over the device.
    ///Implements Drop - drop will return control to the device's firmware
    pub fn new(device_index: DeviceIndex) -> Self
    {
        CoolerMasterDevice
        {
            device_index,
            color_matrix: ColorMatrix{
                key_color: [[KeyColor::new(0,0,0); 24]; 8],
            },
            layout:      unsafe{ GetDeviceLayout(device_index) },
            sdk_version: unsafe{ GetCM_SDK_DllVer() },
            has_led_control: unsafe{ EnableLedControl(true, device_index) },
        }
    }

    ///Returns true if the device is plugged in
    pub fn is_plugged_in(&self) -> bool
    {
        return unsafe { IsDevicePlug(self.device_index) };
    }

    ///Sets the current effect
    pub fn set_effect(&self, effect: EffectIndex)
    {
        unsafe{ SwitchLedEffect(effect, self.device_index) };
    }

    ///Sets all the keys to the given rgb values
    pub fn set_full_color(&self, r: u8, g: u8, b: u8)
    {
        unsafe{ SetFullLedColor(r, g, b, self.device_index) };
    }

    ///Update the lighting with the values in the color_matrix struct
    pub fn update_colors_from_matrix(&self) -> Result<(), ()>
    {
        return match unsafe { SetAllLedColor(self.color_matrix, self.device_index) }
        {
            true => Ok(()),
            false => Err(())
        }
    }

    ///Tries to obtain/drop led control.
    pub fn set_led_control(&mut self, enable: bool) -> Result<(), ()>
    {
        return match unsafe { EnableLedControl(enable, self.device_index) }
        {
            true => Ok(()),
            false => Err(())
        }
    }

    ///Returns true if the leds are controlled by this software instead of the device's firmware.
    pub fn get_led_control(&self) -> bool
    {
        return self.has_led_control;
    }
}

impl Drop for CoolerMasterDevice {
    fn drop(&mut self)
    {
        unsafe { EnableKeyInterrupt(false, self.device_index) };
        unsafe { EnableLedControl(false, self.device_index) };
    }
}

#[cfg(test)]
mod test
{
    use ffi::*;
    use CoolerMasterDevice;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_ffi()
    {
        let version = unsafe{ GetCM_SDK_DllVer() };
        println!("version from c api: {}", version);

        unsafe
        {
            EnableLedControl(true, DeviceIndex::SK621);
            SetFullLedColor(0,0,0, DeviceIndex::SK621);

            let mut color_matrix = ColorMatrix{
                key_color: [[KeyColor::new(0,0,0); 24]; 8],
            };

            for row in 0..8
            {
                for column in 0..24
                {
                    color_matrix.key_color[row][column].r = 255;
                    color_matrix.key_color[row][column].g = 255;
                    color_matrix.key_color[row][column].b = 255;
                    SetAllLedColor(color_matrix, DeviceIndex::SK621);
                    sleep(Duration::from_millis(100));
                }
            }

            SwitchLedEffect(EffectIndex::Breath, DeviceIndex::SK621);
            sleep(Duration::from_secs(5));
        }
    }

    #[test]
    fn test_device()
    {
        let mut device = CoolerMasterDevice::new(DeviceIndex::SK621);
        println!("version from device: {}", device.sdk_version);

        device.set_full_color(0,0,0);

        for row in 0..8
        {
            for column in 0..24
            {
                device.color_matrix.key_color[row][column].r = 255;
                device.color_matrix.key_color[row][column].g = 255;
                device.color_matrix.key_color[row][column].b = 255;
                device.update_colors_from_matrix().unwrap();
                sleep(Duration::from_millis(100));
            }
        }

        device.set_effect(EffectIndex::Breath);
        sleep(Duration::from_secs(5));
    }
}

