# Arduino Uno Accelaration reader in Rust

## Components

- Arduino Uno (Probably possible with other AVR microcontrollers)
- [BMX055](https://akizukidenshi.com/catalog/g/gK-13010/) (Japanese website)

### Datasheet
- [BMX055](https://akizukidenshi.com/download/ds/bosch/BST-BMX055-DS000.pdf)

## Environment

- Windows 11
- rustc 1.68.0-nightly (37d7de337 2022-12-12)
- cargo 1.68.0-nightly (70898e522 2022-12-05)
- ravedude v0.1.5 (no git)

## Setup

### Hardware

| Arduino Uno | BMX055 |
| ----------- | ------ |
| 5V          | VCC    |
| GND         | GND    |
| A4          | SDA    |
| A5          | SCL    |

### Software

```bash
cargo install ravedude
```

For details to `avr-hal` repository in [GitHub](https://github.com/Rahix/avr-hal).

## Usage

```bash
cargo run --release
```

### Output Examples

```bash
Start initializing
Finish sensor initializing
Start loop
ACCL -> X: 0.24412, Y: 0.11718, Z: 9.81382
GYRO -> X: 0.01304, Y: 0.00759, Z: 0.02083
ACCL -> X: 0.23436, Y: 0.11718, Z: 9.81382
GYRO -> X: 0.00146, Y: 0.00885, Z: 0.00139
```

## Future outlook

- [ ] Use magnetic sensors
- [ ] Implement Madgwick filter
- [ ] Sensor fusion for posture estimation
