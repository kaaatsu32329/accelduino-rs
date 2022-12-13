use arduino_hal as hal;

use core::ops::Range;
use hal::{prelude::*, I2c};
use quaternion_core::Vector3;
use serde::{Deserialize, Serialize};

use crate::traits::sensor::Sensor;
use crate::traits::{accl::Accl, gyro::Gyro, mag::Mag};

const ADDR_ACCL: u8 = 0x19;
const ADDR_GYRO: u8 = 0x69;
// const ADDR_MAG: u8 = 0x13;

const ACCL_RANGE_BUFFER: [u8; 2] = [0x0F, 0x03];
const ACCL_BANDWIDTH_BUFFER: [u8; 2] = [0x10, 0x08];
const ACCL_MODE_BUFFER: [u8; 2] = [0x11, 0x00];
const GYRO_RANGE_BUFFER: [u8; 2] = [0x0F, 0x04];
const GYRO_BANDWIDTH_BUFFER: [u8; 2] = [0x10, 0x07];
const GYRO_MODE_BUFFER: [u8; 2] = [0x11, 0x00];
// const MAG_RESET1_BUFFER: [u8; 2] = [0x4B, 0x83];
// const MAG_RESET2_BUFFER: [u8; 2] = [0x4B, 0x01];
// const MAG_MODE_BUFFER: [u8; 2] = [0x4C, 0x00];
// const MAG_AXIS_BUFFER: [u8; 2] = [0x4E, 0x84];
// const MAG_REP_XY_BUFFER: [u8; 2] = [0x51, 0x04];
// const MAG_REP_Z_BUFFER: [u8; 2] = [0x52, 0x16];

const DATA: Range<usize> = 0..6;
const CORRECTION: f32 = 0.0097656;

#[derive(Deserialize, Serialize)]
pub struct Bmx055 {
    #[serde(skip)]
    i2c: Option<I2c>,
    pub accl: Vector3<f32>,
    pub raw_accl: Vector3<i16>,
    pub gyro: Vector3<f32>,
    pub raw_gyro: Vector3<i16>,
    pub mag: Vector3<f32>,
    pub raw_mag: Vector3<i16>,
}

impl Bmx055 {
    pub fn new(i2c: I2c) -> Self {
        Self {
            i2c: Some(i2c),
            accl: [0f32; 3],
            raw_accl: [0i16; 3],
            gyro: [0f32; 3],
            raw_gyro: [0i16; 3],
            mag: [0f32; 3],
            raw_mag: [0i16; 3],
        }
    }
}

impl Sensor for Bmx055 {
    fn init(&mut self) {
        // Initialize acceleration
        self.i2c
            .as_mut()
            .unwrap()
            .write(ADDR_ACCL, &ACCL_RANGE_BUFFER)
            .unwrap();
        hal::delay_ms(100);
        self.i2c
            .as_mut()
            .unwrap()
            .write(ADDR_ACCL, &ACCL_BANDWIDTH_BUFFER)
            .unwrap();
        hal::delay_ms(100);
        self.i2c
            .as_mut()
            .unwrap()
            .write(ADDR_ACCL, &ACCL_MODE_BUFFER)
            .unwrap();
        // Initialize gyro
        hal::delay_ms(100);
        self.i2c
            .as_mut()
            .unwrap()
            .write(ADDR_GYRO, &GYRO_RANGE_BUFFER)
            .unwrap();
        hal::delay_ms(100);
        self.i2c
            .as_mut()
            .unwrap()
            .write(ADDR_GYRO, &GYRO_BANDWIDTH_BUFFER)
            .unwrap();
        hal::delay_ms(100);
        self.i2c
            .as_mut()
            .unwrap()
            .write(ADDR_GYRO, &GYRO_MODE_BUFFER)
            .unwrap();
        hal::delay_ms(100);

        /*
        // Initialize magnetic
        self.i2c
            .as_mut()
            .unwrap()
            .write(ADDR_MAG, &MAG_RESET1_BUFFER)
            .unwrap();
        hal::delay_ms(100);

        self.i2c
            .as_mut()
            .unwrap()
            .write(ADDR_MAG, &MAG_RESET2_BUFFER)
            .unwrap();
        hal::delay_ms(100);

        self.i2c
            .as_mut()
            .unwrap()
            .write(ADDR_MAG, &MAG_MODE_BUFFER)
            .unwrap();
        hal::delay_ms(100);

        self.i2c
            .as_mut()
            .unwrap()
            .write(ADDR_MAG, &MAG_AXIS_BUFFER)
            .unwrap();
        hal::delay_ms(100);

        self.i2c
            .as_mut()
            .unwrap()
            .write(ADDR_MAG, &MAG_REP_XY_BUFFER)
            .unwrap();
        hal::delay_ms(100);

        self.i2c
            .as_mut()
            .unwrap()
            .write(ADDR_MAG, &MAG_REP_Z_BUFFER)
            .unwrap();
        hal::delay_ms(100);
         */
    }
}

impl Accl<f32> for Bmx055 {
    fn read_accl(&mut self) {
        let mut data = [0u8; 6];
        for i in DATA {
            let buf = [2 + i as u8; 1];
            self.i2c.as_mut().unwrap().write(ADDR_ACCL, &buf).unwrap();

            self.i2c
                .as_mut()
                .unwrap()
                .read(ADDR_ACCL, &mut data[i..i + 1])
                .unwrap();
        }
        let x = ((data[1] as i16 * 256) + data[0] as i16) / 16;
        let y = ((data[3] as i16 * 256) + data[2] as i16) / 16;
        let z = ((data[5] as i16 * 256) + data[4] as i16) / 16;

        self.raw_accl[0] = x;
        self.raw_accl[1] = y;
        self.raw_accl[2] = z;

        self.accl[0] = x as f32 * CORRECTION;
        self.accl[1] = y as f32 * CORRECTION;
        self.accl[2] = z as f32 * CORRECTION;
    }

    fn get_accl(&self) -> Vector3<f32> {
        self.accl
    }
}

impl Gyro<f32> for Bmx055 {
    fn read_gyro(&mut self) {
        todo!()
    }

    fn get_gyro(&self) -> Vector3<f32> {
        self.gyro
    }
}

impl Mag<f32> for Bmx055 {
    fn read_mag(&mut self) {
        todo!()
    }

    fn get_mag(&self) -> Vector3<f32> {
        self.mag
    }
}
