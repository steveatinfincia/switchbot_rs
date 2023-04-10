/* 
 * This module parses SwitchBot Bluetooth LE advertisement data. It does not
 * currently support constructing commands to send to SwitchBot devices, but
 * that is a planned feature.
 * 
 * To use this code, it is advised that you connect to each BLE device, discover
 * services to look for the SwitchBot primary service UUID, and only attempt
 * to parse the service/manufacturer data once it is confirmed that it is a
 * SwitchBot device.
 * 
 * It is not advised to use the SwitchBot primary service UUID to *filter*
 * discovered devices while scanning, even though most platforms allow it,
 * because SwitchBot devices do not always include that service UUID in the
 * Bluetooth LE advertisement broadcast. Specifically the Plug and MeterPlus 
 * are known not to advertise any primary service UUID, however the Bot device
 * does and others may as well.
 * 
 * Some SwitchBot devices also use a different manufacturer ID in Bluetooth LE
 * ADV_IND broadcasts (Woan Technology has a registered ID 0x0969, but some
 * of their devices use Nordic's 0x0059 instead). It is possible to know
 * in advance which one to look for because the service data will tell us in
 * advance which model we have discovered, and that tells us which manufacturer
 * ID to look for.
 *
 * Lastly, it is possible to parse the advertisement data passively, without
 * connecting to a device to verify which services it offers. In that case,
 * you are making an assumption that the combination of the first service
 * data byte (which SwitchBot devices use to identify each model), along with
 * the length of either the service data or the manufacturer data, will be unique
 * to a particular SwitchBot device.
 * 
 * Obviously this is not guaranteed, but in practice it works and SwitchBot's
 * official libraries seem to work this way.
 * 
 */

use crate::model::{SwitchBotDeviceModel, SwitchBotData};

/*
 * Parse the first byte of a Bluetooth LE Service Data
 * field in SCAN_RSP to get the SwitchBot device model.
 * 
 * Returning Some from this function DOES NOT guarantee
 * that the packet came from a SwitchBot device, it's only
 * a single byte so there is a high chance of false positives.
 * 
 * See switchbot::decode_data() below for details.
 * 
 */
pub fn decode_model(data: u8) -> Option<SwitchBotDeviceModel> {
    match (data & 0b01111111) as i32 {
        0x42 => {
            Some(SwitchBotDeviceModel::Button)
        }
        0x46 => {
            Some(SwitchBotDeviceModel::FanAdd)
        }
        0x48 => {
            Some(SwitchBotDeviceModel::Bot)
        }
        0x4C => {
            Some(SwitchBotDeviceModel::HubAdd)
        }
        0x4D => {
            Some(SwitchBotDeviceModel::HubMiniAdd)
        }
        0x50 => {
            Some(SwitchBotDeviceModel::HubPlusAdd)
        }
        0x54 => {
            Some(SwitchBotDeviceModel::Meter)
        }
        0x63 => {
            Some(SwitchBotDeviceModel::Curtain)
        }
        0x64 => {
            Some(SwitchBotDeviceModel::ContactSensor)
        }
        0x65 => {
            Some(SwitchBotDeviceModel::Humidifier)
        }
        0x66 => {
            Some(SwitchBotDeviceModel::Fan)
        }
        0x67 => {
            Some(SwitchBotDeviceModel::PlugMiniUS)
        }
        0x69 => {
            Some(SwitchBotDeviceModel::MeterPlus)
        }
        0x6A => {
            Some(SwitchBotDeviceModel::PlugMiniJP)
        }
        0x6C => {
            Some(SwitchBotDeviceModel::Hub)
        }
        0x6D => {
            Some(SwitchBotDeviceModel::HubMini)
        }
        0x6F => {
            Some(SwitchBotDeviceModel::SmartLock)
        }
        0x70 => {
            Some(SwitchBotDeviceModel::HubPlus)
        }
        0x72 => {
            Some(SwitchBotDeviceModel::LEDStripLight)
        }
        0x73 => {
            Some(SwitchBotDeviceModel::MotionSensor)
        }
        0x74 => {
            Some(SwitchBotDeviceModel::MeterAdd)
        }
        0x75 => {
            Some(SwitchBotDeviceModel::ColorBulb)
        }
        _ => {
            None
        }
    }
}

/*
 * Decode BLE advertisment data from a SwitchBot device.
 * 
 * The location of the data depends on the device model, which is determined
 * by the first byte of the service_data in SCAN_RSP. 
 * 
 * However, finding a valid device model identifier does NOT guarantee that the
 * device is actually a SwitchBot device; a single byte can only have 256 distinct
 * values, which means there is a significant chance of false positives.
 * 
 * There is an additional check below to reduce those false positives: the length of
 * the service data or manufacturer data fields. However, that isn't conclusive. 
 * 
 * In practice you should connect to the device and determine if it provides the
 * primary SwitchBot service UUID, and only then rely on the values.
 *
 */
pub fn decode_data(service_data: &[u8],
                   manufacturer_data: Option<&[u8]>) -> (Option<SwitchBotDeviceModel>, 
                                                         Option<SwitchBotData>) {
    /*
     * No SwitchBot device broadcasts a SCAN_RSP packet with a service data
     * field smaller than 3 bytes. Skipping those packets will reduce the
     * chances that devices from other manufacturers will look like SwitchBot
     * devices.
     */
    if service_data.len() < 3 {
        return (None, None);
    }

    let Some(model) = decode_model(service_data[0]) else {
        return (None, None);
    };

    match model {
        SwitchBotDeviceModel::Bot => {            
            if service_data.len() != 3 {
                println!("Found SwitchBotDevice::Bot but service data length invalid: {}", service_data.len());
                return (None, None);
            }
            
            let state: bool = if service_data[1] & 0b01000000 == 0b01000000 { true } else { false };
            
            let switchbot_data = SwitchBotData::Bot {
                battery: service_data[2] & 0b01111111,
                state: state,
            };
            
            return (Some(model), Some(switchbot_data));
        }
        SwitchBotDeviceModel::Meter | SwitchBotDeviceModel::MeterPlus => {            
            if service_data.len() != 6 {
                println!("Found SwitchBotDevice::Meter but service data length invalid: {}", service_data.len());
                return (None, None);
            }

            let temp_sign: i32 = if service_data[4] & 0b10000000 == 0b10000000 { 1 } else  { -1 };
            let temp_c: i32 = temp_sign * ((service_data[4] & 0b01111111) as i32 + (service_data[3] & 0b00001111) as i32 / 10);
            
            let switchbot_data = SwitchBotData::Meter {
                temperature: temp_c,
                humidity: service_data[5] & 0b01111111,
                battery: service_data[2] & 0b01111111,
            };
            
            return (Some(model), Some(switchbot_data));
        }
        SwitchBotDeviceModel::Humidifier => {
            if service_data.len() != 5 {
                println!("Found SwitchBotDevice::Humidifier but service data length invalid: {}", service_data.len());
                return (None, None);
            }
            
            let state: bool = if service_data[1] & 0b10000000 == 0b10000000 { true } else { false };
            let auto_mode: bool= if service_data[4] & 0b10000000 == 0b10000000 { true } else { false };
            let humidity_setting: u8 = service_data[4] & 0b01111111;
            
            let switchbot_data = SwitchBotData::Humidifier {
                humidity: humidity_setting,
                state: state,
                auto_mode: auto_mode,
            };
            
            return (Some(model), Some(switchbot_data));
        }
        SwitchBotDeviceModel::PlugMiniUS | SwitchBotDeviceModel::PlugMiniJP => {        
            let Some(manufacturer_data) = manufacturer_data else {
                return (None, None);
            };

            /*
             * Number takes into account the fact that the manufacturingData buffer is a map, not raw. The
             * first 2 bytes are part of the manufacturing data ID which the bluez-async API consumes as a
             * HashMap key.
             * 
             */
            if manufacturer_data.len() != 12 {
                println!("Found SwitchBotDevicePlugMini::US|JP but service data length invalid: {}", manufacturer_data.len());
                return (None, None);
            }
            
            let state: bool = if manufacturer_data[7] == 0x80 { true } else { false };
            let overload: bool = (manufacturer_data[10] & 0b10000000) == 0b10000000;
            let watts: i16 = (((manufacturer_data[10] as i16 & 0b01111111) << 8) + manufacturer_data[11] as i16) / 10;
            
            let sensor_data = SwitchBotData::Plug {
                wifi_rssi: -(manufacturer_data[9] as i16),
                state: state,
                watts: watts,
                overload: overload
            };
            
            return (Some(model), Some(sensor_data))
        }
        _ => {
            return (None, None)
        }
    };
}

