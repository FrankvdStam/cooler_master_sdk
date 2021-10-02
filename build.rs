#[cfg(target_pointer_width = "64")]
const COOLER_MASTER_SDK_PATH: &str = r"src/CoolerMasterSdk_v28/SDK/x64";
#[cfg(target_pointer_width = "32")]
const COOLER_MASTER_SDK_PATH: &str = r"src/CoolerMasterSdk_v28/SDK/x86";

fn main()
{
    //Statically link the cooler master sdk's .lib file
    println!("cargo:rustc-link-search={}", COOLER_MASTER_SDK_PATH);
    println!("cargo:rustc-link-lib=static=SDKDLL");
}