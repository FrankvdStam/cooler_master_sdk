

extern crate libc;


#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeyColor
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl KeyColor
{
    pub fn zeroed() -> Self
    {
        KeyColor{r: 0, g: 0, b: 0}
    }

    pub fn new(r: u8, g: u8, b: u8) -> Self
    {
        KeyColor{r, g, b}
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ColorMatrix
{
    pub key_color: [[KeyColor; 24]; 8],
}

#[repr(C)]
pub enum EffectIndex
{
    FullOn = 0,
    Breath = 1,
    BreathCycle = 2,
    Single = 3,
    Wave = 4,
    Ripple = 5,
    Cross = 6,
    Rain = 7,
    Star = 8,
    Snake = 9,
    Rec = 10,
    Spectrum = 11,
    RapidFire = 12,
    Indicator = 13,
    FireBall = 14,
    WaterRipple = 15,
    ReactivePunch = 16,
    Snowing = 17,
    HeartBeat = 18,
    ReactiveTornado = 19,
    Multi1 = 0xE0,
    Multi2 = 0xE1,
    Multi3 = 0xE2,
    Multi4 = 0xE3,
    Off = 0xFE
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub enum DeviceIndex
{
    MKeys_L         = 0,
    MKeys_S         = 1,
    MKeys_L_White   = 2,
    MKeys_M_White   = 3,
    MMouse_L        = 4,
    MMouse_S        = 5,
    MKeys_M         = 6,
    MKeys_S_White   = 7,
    MM520           = 8,
    MM530           = 9,
    MK750           = 10,
    CK372           = 11,
    CK550_552       = 12,
    CK551           = 13,
    MM830           = 14,
    CK530           = 15,
    MK850           = 16,
    SK630           = 17,
    SK650           = 18,
    SK621           = 19,
    MK730           = 20,
    SK630_White     = 21,
    SK650_White     = 22,
    SK621_White     = 23,
    MM711           = 24,
    MM831           = 25,
    DEFAULT         = 0xFFFF
}

//Enumeration of device layout
#[repr(C)]
pub enum KeyboardLayout
{
    Uninit = 0,
    US = 1,
    EU = 2,
    JP = 3}


//typedef void (CALLBACK * KEY_CALLBACK)(int iRow, int iColumn, bool bPressed);

type KeyCallback = extern "C" fn(row: libc::c_int, column: libc::c_int, pressed: bool);

#[link(name = "SDKDLL")]
extern
{
    ///Get SDK Dll's Version
    pub fn GetCM_SDK_DllVer() -> libc::c_int;

    //Maybe just ignore the existence of this - no need for all the risky pointer business when there are idiomatic alternatives for getting the time..
    //pub fn GetNowTime() -> *mut ffi::libc::wchar_t;

    ///obtain current CPU usage ratio 0 ~ 100
    pub fn fnGetNowCPUUsage(p_error_code: *mut u32) -> libc::c_long;

    ///Obtain current RAM usage ratio 0 ~ 100
    pub fn GetRamUsage() -> u32;

    ///Obtain current volume  0 ~ 1 float
    pub fn GetNowVolumePeekValue() -> libc::c_float;

    ///set operating device
    pub fn SetControlDevice(dev_index: DeviceIndex);

    ///verify if the deviced is plugged in
    pub fn IsDevicePlug(dev_index: DeviceIndex) -> bool;

    ///Obtain current device layout
    pub fn GetDeviceLayout(dev_index: DeviceIndex) -> KeyboardLayout;

    ///set control over device’s LED - enable: true Controlled by SW，false Controlled by FW
    pub fn EnableLedControl(enable: bool, dev_index: DeviceIndex) -> bool;

    ///switch device current effect
    pub fn SwitchLedEffect(effect_index: EffectIndex, dev_index: DeviceIndex) -> bool;

    ///Print out the lights setting from Buffer to LED - auto: false means manual, call this function once, then print out once; true means auto, any light update will print out directly
    pub fn RefreshLed(auto: bool, dev_index: DeviceIndex) -> bool;

    ///set entire keyboard LED color
    pub fn SetFullLedColor(r: u8, g: u8, b: u8, dev_index: DeviceIndex) -> bool;

    /// Set Keyboard "every LED" color - COLOR_MATRIX colorMatrix:structure，fill up RGB value according to LED Table
    pub fn SetAllLedColor(color_matrix: ColorMatrix, dev_index: DeviceIndex) -> bool;

    ///Set single Key LED color
    pub fn SetLedColor(row: libc::c_int, column: libc::c_int, r: u8, g: u8, b: u8, dev_index: DeviceIndex) -> bool;

    ///To enable the call back function
    pub fn EnableKeyInterrupt(enable: bool, dev_index: DeviceIndex) -> bool;

    ///Setup the call back function of button
    pub fn  SetKeyCallBack(callback: KeyCallback, dev_index: DeviceIndex);
}
