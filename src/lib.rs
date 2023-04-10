pub mod constant;
pub mod protocol;
pub mod model;

pub use crate::protocol::{decode_model, decode_data};
pub use crate::model::{SwitchBotDeviceModel, SwitchBotData};

pub use crate::constant::{SWITCHBOT_WOAN_MANUFACTURER_ID, SWITCHBOT_NORDIC_MANUFACTURER_ID};
pub use crate::constant::{SWITCHBOT_SERV_UUID_PRIMARY, SWITCHBOT_SERV_UUID_WOAN_TECHNOLOGY, SWITCHBOT_SERV_UUID_WOAN_TECHNOLOGY2};
pub use crate::constant::{SWITCHBOT_CHAR_UUID_WRITE, SWITCHBOT_CHAR_UUID_NOTIFY};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switchbot_zero_buffer_test() {
        let service_data: [u8; 3] = [0; 3];

        let manufacturer_data: [u8; 12] = [0; 12];

        let (model,
             switchbot_data) = protocol::decode_data(&service_data, Some(&manufacturer_data));

        assert!(model.is_none());
        assert!(switchbot_data.is_none());
    }

    #[test]
    fn switchbot_model_bot_battery_100_state_on_test() -> Result<(), &'static str> {
        let service_data: [u8; 3] = [0x48, 0x40, 0x64];

        let (model,
             switchbot_data) = protocol::decode_data(&service_data, None);

        let Some(SwitchBotDeviceModel::Bot) = model else {
            return Err("invalid model");
        };

        match switchbot_data {
            Some(SwitchBotData::Bot { battery, state }) => {
                assert_eq!(battery, 100);
                assert!(state);

                return Ok(());
            },
            _ => {
                return Err("invalid bot data");
            }
        }
    }

    #[test]
    fn switchbot_model_bot_battery_50_state_off_test() -> Result<(), &'static str> {
        let service_data: [u8; 3] = [0x48, 0x0, 0x32];

        let (model,
             switchbot_data) = protocol::decode_data(&service_data, None);

        let Some(SwitchBotDeviceModel::Bot) = model else {
            return Err("invalid model");
        };

        match switchbot_data {
            Some(SwitchBotData::Bot { battery, state }) => {
                assert_eq!(battery, 50);
                assert!(!state);

                return Ok(());
            },
            _ => {
                return Err("invalid bot data");
            }
        }
    }

    #[test]
    fn switchbot_model_meterplus_battery_100_temperature_23_humidity_42_test() -> Result<(), &'static str> {
        let service_data: [u8; 6] = [0x69, 
                                     0x00,  // ignored on this model
                                     0x64,  // 100% battery level
                                     0x00, 
                                     0x80,  // positive temperature sign in MSB
                                     0x2A]; // 42% humidity];

        let (model,
             switchbot_data) = protocol::decode_data(&service_data, None);

        let Some(SwitchBotDeviceModel::MeterPlus) = model else {
            return Err("invalid model");
        };

        match switchbot_data {
            Some(SwitchBotData::Meter { battery, temperature, humidity }) => {
                assert_eq!(battery, 100);
                assert_eq!(temperature, 23);
                assert_eq!(humidity, 42);

                return Ok(());
            },
            _ => {
                return Err("invalid meter data");
            }
        }
    }
}
