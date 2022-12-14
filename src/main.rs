#![no_std]
#![no_main]

mod bmx055;
mod madgwick_filter;
mod traits;

use arduino_hal as hal;
use panic_halt as _;

use hal::prelude::*;
use traits::{accl::Accl, sensor::Sensor, gyro::Gyro};

use crate::bmx055::Bmx055;

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut serial = hal::default_serial!(dp, pins, 57600);

    let i2c = hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    ufmt::uwriteln!(&mut serial, "Start initializing").void_unwrap();

    let mut accl_sensor = Bmx055::new(i2c);

    accl_sensor.init();
    ufmt::uwriteln!(&mut serial, "Finish sensor initializing").void_unwrap();

    ufmt::uwriteln!(&mut serial, "Start loop").void_unwrap();

    loop {
        accl_sensor.read_accl();
        let _ = accl_sensor.get_accl();

        accl_sensor.read_gyro();
        let _ = accl_sensor.get_gyro();

        let accl_x = ufmt_float::uFmt_f32::Five(accl_sensor.accl[0]);
        let accl_y = ufmt_float::uFmt_f32::Five(accl_sensor.accl[1]);
        let accl_z = ufmt_float::uFmt_f32::Five(accl_sensor.accl[2]);

        let gyro_x = ufmt_float::uFmt_f32::Five(accl_sensor.gyro[0]);
        let gyro_y = ufmt_float::uFmt_f32::Five(accl_sensor.gyro[1]);
        let gyro_z = ufmt_float::uFmt_f32::Five(accl_sensor.gyro[2]);

        ufmt::uwriteln!(&mut serial, "ACCL -> X: {}, Y: {}, Z: {}", accl_x, accl_y, accl_z).void_unwrap();
        ufmt::uwriteln!(&mut serial, "GYRO -> X: {}, Y: {}, Z: {}", gyro_x, gyro_y, gyro_z).void_unwrap();

        hal::delay_ms(1000);
    }
}
