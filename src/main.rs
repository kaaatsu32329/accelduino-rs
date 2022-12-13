#![no_std]
#![no_main]

mod bmx055;
mod traits;

use arduino_hal as hal;
use panic_halt as _;

use hal::prelude::*;
use traits::{accl::Accl, sensor::Sensor};

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

        let x = ufmt_float::uFmt_f32::Five(accl_sensor.accl[0]);
        let y = ufmt_float::uFmt_f32::Five(accl_sensor.accl[1]);
        let z = ufmt_float::uFmt_f32::Five(accl_sensor.accl[2]);

        ufmt::uwriteln!(&mut serial, "X: {}, Y: {}, Z: {}", x, y, z).void_unwrap();

        hal::delay_ms(1000);
    }
}
