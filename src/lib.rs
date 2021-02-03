// #![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]

use embedded_hal::digital::v2::OutputPin;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct Dac<SPI, NSS, LDAC, ENABLE> {
    spi: SPI,
    nss: NSS,
    ldac: LDAC,
    enable: ENABLE,
    active: bool,
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(PartialEq)]
pub enum Channel {
    A = 0b0000,
    B = 0b0010,
    C = 0b0100,
    D = 0b0110,
    ALL = 0b0111,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum DacError {
    /// Unable to write to bus
    BusWriteError,
}

#[allow(dead_code)]
pub fn index_to_channel(val: u8) -> Channel {
    if val == 0 {
        return Channel::A;
    } else if val == 1 {
        return Channel::B;
    } else if val == 2 {
        return Channel::C;
    } else if val == 3 {
        return Channel::D;
    }
    panic!("Channel unknown for index {}", val);
}

#[allow(dead_code)]
fn get_payload(channel: Channel, value: u16) -> [u8; 3] {
    let mut command: [u8; 3] = [0; 3];

    // Channel select
    // a 0b00010000
    // b 0b00010010
    // c 0b00010100
    // d 0b00010110
    command[0] = 0b00010000 | (channel as u8);
    // Upper 8 bits
    command[1] = ((value & 0xFF00) >> 8) as u8;
    // Lower 8 bits
    command[2] = (value & 0xFF) as u8;

    command
}

impl<SPI, NSS, LDAC, ENABLE> Dac<SPI, NSS, LDAC, ENABLE>
where
    NSS: OutputPin,
    LDAC: OutputPin,
    ENABLE: OutputPin,
    SPI: embedded_hal::blocking::spi::Write<u8>,
{
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enable.set_low();
        self.nss.set_low();
        self.nss.set_high();
        self.enable.set_high();

        // Rising edge to reset the DAC registers
        self.ldac.set_low();

        // Wait
        let mut x = 0;
        while x < 10000 {
            x += 1;
        }
        self.ldac.set_high();

        // Wait
        x = 0;
        while x < 10000 {
            x += 1;
        }

        self.ldac.set_low();

        self.active = true;
    }

    #[allow(dead_code)]
    pub fn set_value(&mut self, channel: Channel, value: u16) -> Result<(), DacError> {
        if !self.active {
            return Ok(());
        }

        let command: [u8; 3] = get_payload(channel, value);
        self.write(&command)
    }

    #[allow(dead_code)]
    fn write(&mut self, values: &[u8]) -> Result<(), DacError> {
        self.enable.set_low();
        self.nss.set_low();
        let result = self.spi.write(values);
        self.nss.set_high();
        self.enable.set_high();

        match result {
            Ok(v) => Ok(v),
            Err(_e) => Err(DacError::BusWriteError),
        }
    }
}
