### Switchbot-rs

A library for interacting with SwitchBot devices via Bluetooth LE.

The code is currently able to decode Bluetooth LE service data packets for the
bot, meter/plus, humidifier, and plug mini devices.

There are tests for a few specific cases, written using manually captured
data from real SwitchBot devices.

The `constant` module contains a few useful values, but the process of
connecting to a Bluetooth LE device is not specific to SwitchBot devices,
so this crate is solely intended for processing the data and does not
contain code for connecting to actual devices.

Determining which device to connect to and handling those connections
is the responsibility of another crate, you can see an example of
how to do that in `ble_bridge_rs`.
