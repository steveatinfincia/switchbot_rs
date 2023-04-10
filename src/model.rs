#[derive(Copy, Clone, Debug)]
pub enum SwitchBotData {
    Bot { battery: u8,  state: bool },
    Meter { battery: u8, temperature: i32, humidity: u8 },
    Plug { wifi_rssi: i16, state: bool, watts: i16, overload: bool },
    Humidifier { state: bool, humidity: u8, auto_mode: bool },
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum SwitchBotDeviceModel {
    Button = 0x42,
    FanAdd = 0x46,
    Bot = 0x48,
    HubAdd = 0x4C,
    HubMiniAdd = 0x4D,
    HubPlusAdd = 0x50,
    Meter = 0x54,
    Curtain = 0x63,
    ContactSensor = 0x64,
    Humidifier = 0x65,
    Fan = 0x66,
    PlugMiniUS = 0x67,
    MeterPlus = 0x69,
    PlugMiniJP = 0x6A,
    Hub = 0x6C,
    HubMini = 0x6D,
    SmartLock = 0x6F,
    HubPlus = 0x70,
    LEDStripLight = 0x72,
    MotionSensor = 0x73,
    MeterAdd = 0x74,
    ColorBulb = 0x75,
}
