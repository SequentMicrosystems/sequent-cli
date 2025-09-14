use crate::{Error, Result};

use embedded_hal::i2c::I2c;
use linux_embedded_hal::I2cdev; // Import the I2c trait for write_read

pub trait CardInfo {
    const BASE_ADDR: u8;
    const MAX_STACK_LEVEL: u8 = 7; // Maximum stack level supported
    const ASCENDING_ADDRS: bool = true; // Whether addresses increase with stack level
    const CARD_NAME: &'static str;
    const PROGRAM_NAME: &'static str;
    const VERSION: &'static str;

    fn stack_level(&self) -> u8;
}

pub trait Card {
    fn base_addr(&self) -> u8;
    fn max_stack_level(&self) -> u8;
    fn addr(&self) -> u8;
    fn card_name(&self) -> &'static str;
    fn program_name(&self) -> &'static str;
    fn version(&self) -> &'static str;

    fn read_n_bytes(&self, register: u8, n: u8) -> Result<Vec<u8>>;

    fn read_u_n(&self, register: u8, n: u8) -> Result<u32> {
        let bytes = self.read_n_bytes(register, n)?;
        println!("DEBUG: {bytes:?}");
        let mut val = 0u32;
        for (i, b) in bytes.iter().enumerate() {
            val |= (*b as u32) << (i * 8);
        }
        Ok(val)
    }

    fn read_i_n(&self, register: u8, n: u8) -> Result<i32> {
        let val = self.read_u_n(register, n)?;
        let sign_bit = 1 << (n * 8 - 1);
        if val & sign_bit != 0 {
            Ok((val as i32) - (1 << (n * 8)))
        } else {
            Ok(val as i32)
        }
    }

    fn read_f_n(&self, register: u8, n: u8) -> Result<f32> {
        // TODO: IMPLEMENT ME
        Ok(0.)
    }

    fn read_u8(&self, register: u8) -> Result<u8>;
    //fn read_u16(&self, register: u8) -> Result<u16>;
    fn read_float(&self, register: u8) -> Result<f32>;

    fn write_bytes(&self, register: u8, value: &[u8]) -> Result<()>;

    fn write_u8(&self, register: u8, value: u8) -> Result<()> {
        let mut i2c = I2cdev::new("/dev/i2c-1")?;
        i2c.write(Self::base_addr(self) as u16, &[register, value])?;
        Ok(())
    }

    fn write_u_n(&self, register: u8, n: u8, value: u32) -> Result<()> {
        let bytes: Vec<u8> = (0..n).map(|i| ((value >> (i * 8)) & 0xff) as u8).collect();
        self.write_bytes(register, &bytes)
    }

    fn write_bit(&self, register: u8, bit: u8, state: bool) -> Result<()>;
}

impl<T> Card for T
where
    T: CardInfo,
{
    fn base_addr(&self) -> u8 {
        Self::BASE_ADDR
    }
    fn max_stack_level(&self) -> u8 {
        Self::MAX_STACK_LEVEL
    }
    fn addr(&self) -> u8 {
        // TODO: Add if ascending
        Self::BASE_ADDR + self.stack_level()
    }
    fn card_name(&self) -> &'static str {
        Self::CARD_NAME
    }
    fn program_name(&self) -> &'static str {
        Self::PROGRAM_NAME
    }
    fn version(&self) -> &'static str {
        Self::VERSION
    }
    fn read_n_bytes(&self, register: u8, n: u8) -> Result<Vec<u8>> {
        let mut i2c = I2cdev::new("/dev/i2c-1")?;
        let mut b = vec![0u8; n.into()];
        i2c.write_read(self.addr() as u16, &[register], &mut b)?;
        Ok(b)
    }
    fn read_float(&self, register: u8) -> Result<f32> {
        let mut i2c = I2cdev::new("/dev/i2c-1")?;
        let mut b = [0u8; 4];
        i2c.write_read(self.addr() as u16, &[register], &mut b)?;
        let val = f32::from_le_bytes(b);
        Ok(val)
    }
    /*
    fn read_u16(&self, register: u8) -> Result<u16> {
        let mut i2c = I2cdev::new("/dev/i2c-1")?;
        let mut b = [0u8; 2];
        i2c.write_read(Self::BASE_ADDR as u16, &[register], &mut b)?;
        Ok((b[1] as u16) << 8 | (b[0] as u16))
    }
    */
    fn read_u8(&self, register: u8) -> Result<u8> {
        let mut i2c = I2cdev::new("/dev/i2c-1")?;
        let mut b = [0u8; 1];
        i2c.write_read(self.addr() as u16, &[register], &mut b)?;
        Ok(b[0])
    }
    fn write_bytes(&self, register: u8, value: &[u8]) -> Result<()> {
        let mut i2c = I2cdev::new("/dev/i2c-1")?;
        let mut data = vec![register];
        data.extend_from_slice(value);
        i2c.write(self.addr() as u16, &data)?;
        Ok(())
    }

    fn write_bit(&self, register: u8, bit: u8, state: bool) -> Result<()> {
        if bit > 7 {
            return Err(Error::InvalidParameter {
                param: "bit inside write_bit".to_string(),
                value: bit.to_string(),
            });
        }
        let mut val = self.read_u8(register)?;
        if state {
            val |= 1 << bit;
        } else {
            val &= !(1 << bit);
        }
        self.write_u8(register, val)?;
        Ok(())
    }
}
