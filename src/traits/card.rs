use crate::{Error, Result};

use clap::Command;
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
    fn set_stack_level(&mut self, stack_level: u8);
}

pub trait Card {
    fn base_addr(&self) -> u8;
    fn max_stack_level(&self) -> u8;
    fn addr(&self) -> u8;
    fn card_name(&self) -> &'static str;
    fn program_name(&self) -> &'static str;
    fn version(&self) -> &'static str;

    fn read_u8(&self, register: u8) -> Result<u8>;

    fn read_n_bytes(&self, register: u8, n: u8) -> Result<Vec<u8>>;

    fn read_u_n(&self, register: u8, n: u8) -> Result<u32> {
        let bytes = self.read_n_bytes(register, n)?;
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

    fn read_f_n(&self, register: u8, n: u8) -> Result<f64> {
        let bytes = self.read_n_bytes(register, n)?;
        let mut val: u64 = 0;

        // build integer representation from little-endian bytes
        for (i, b) in bytes.iter().enumerate() {
            val |= (*b as u64) << (i * 8);
        }

        // interpret based on size
        let f = match n {
            4 => f32::from_le_bytes(val.to_le_bytes()[..4].try_into().unwrap()) as f64,
            8 => f64::from_le_bytes(val.to_le_bytes()),
            _ => panic!("unsupported float size: {}", n),
        };
        Ok(f)
    }

    fn write_bit(&self, register: u8, bit: u8, state: bool) -> Result<()>;

    fn write_u8(&self, register: u8, value: u8) -> Result<()> {
        let mut i2c = I2cdev::new("/dev/i2c-1")?;
        i2c.write(Self::base_addr(self) as u16, &[register, value])?;
        Ok(())
    }

    fn write_bytes(&self, register: u8, value: &[u8]) -> Result<()>;

    fn write_u_n(&self, register: u8, n: u8, value: u32) -> Result<()> {
        let bytes: Vec<u8> = (0..n).map(|i| ((value >> (i * 8)) & 0xff) as u8).collect();
        self.write_bytes(register, &bytes)
    }

    /*
    //Consider this version instead
    fn write_u_n_(&self, register: u8, n: u8, value: u32) -> Result<()> {
        self.write_bytes(register, &value.to_le_bytes()[0..n as usize])
    }
    */

    fn write_i_n(&self, register: u8, n: u8, value: i32) -> Result<()> {
        let uvalue = if value < 0 {
            (1u32 << (n * 8)) + (value as u32)
        } else {
            value as u32
        };
        self.write_u_n(register, n, uvalue)
    }

    // fn write_i_n
    // fn write_f_n

    fn info_cmd(&self) -> Command {
        Command::new("info").about("Show device information")
    }
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
    fn read_u8(&self, register: u8) -> Result<u8> {
        let mut i2c = I2cdev::new("/dev/i2c-1")?;
        let mut b = [0u8; 1];
        i2c.write_read(self.addr() as u16, &[register], &mut b)?;
        Ok(b[0])
    }
    fn read_n_bytes(&self, register: u8, n: u8) -> Result<Vec<u8>> {
        let mut i2c = I2cdev::new("/dev/i2c-1")?;
        let mut b = vec![0u8; n.into()];
        i2c.write_read(self.addr() as u16, &[register], &mut b)?;
        Ok(b)
    }
    /*
    fn read_float(&self, register: u8) -> Result<f32> {
        let mut i2c = I2cdev::new("/dev/i2c-1")?;
        let mut b = [0u8; 4];
        i2c.write_read(self.addr() as u16, &[register], &mut b)?;
        let val = f32::from_le_bytes(b);
        Ok(val)
    }
    */
    /*
    fn read_u16(&self, register: u8) -> Result<u16> {
        let mut i2c = I2cdev::new("/dev/i2c-1")?;
        let mut b = [0u8; 2];
        i2c.write_read(Self::BASE_ADDR as u16, &[register], &mut b)?;
        Ok((b[1] as u16) << 8 | (b[0] as u16))
    }
    */

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
    fn write_bytes(&self, register: u8, value: &[u8]) -> Result<()> {
        let mut i2c = I2cdev::new("/dev/i2c-1")?;
        let mut data = vec![register];
        data.extend_from_slice(value);
        i2c.write(self.addr() as u16, &data)?;
        Ok(())
    }
}
