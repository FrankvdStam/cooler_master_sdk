#Cooler master sdk
Wraps the official cooler master sdk.
see https://templates.coolermaster.com/

Requires that the cooler master sdk dll can be found via the path environment variable. If the path to SDKDLL can not be found on the path, a runtime error will occur.

#usage
Using the wrapper "CoolerMasterDevice" type:


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

Or if you want to make your own abstraction, use the FFI over the C api directly:

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

I developed this library using CLion, many thanks to Jetbrains for giving me an opensource license for free!