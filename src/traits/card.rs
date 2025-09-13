use linux_embedded_hal::I2cdev;
use embedded_hal::i2c::I2c; // Import the I2c trait for write_read


pub trait CardAddrs {
    const BASE_ADDR: u8;
    const CARD_NAME: &'static str;
    const PROGRAM_NAME: &'static str;
    const VERSION: &'static str;
}

pub trait Card: CardAddrs {
    fn info(&self) -> String {
        format!(
            "{} (CLI Version: {}) at base address 0x{:X}",
            Self::CARD_NAME,
            Self::VERSION,
            Self::BASE_ADDR
        )
    }
    fn read_float(&self, register: u8) -> Result<f32, String> {
        let mut i2c = I2cdev::new("/dev/i2c-1").map_err(|e| e.to_string())?;
        let mut b = [0u8; 4];
        i2c.write_read(Self::BASE_ADDR as u16, &[register], &mut b).map_err(|e| e.to_string())?;
        let val = f32::from_le_bytes(b);
        Ok(val)
    }
    fn read_u16(&self, register: u8) -> Result<u16, String> {
        let mut i2c = I2cdev::new("/dev/i2c-1").map_err(|e| e.to_string())?;
        let mut b = [0u8; 2];
        i2c.write_read(Self::BASE_ADDR as u16, &[register], &mut b).map_err(|e| e.to_string())?;
        Ok((b[1] as u16) << 8 | (b[0] as u16))
    }
    fn read_u8(&self, register: u8) -> Result<u8, String> {
        let mut i2c = I2cdev::new("/dev/i2c-1").map_err(|e| e.to_string())?;
        let mut b = [0u8; 1];
        i2c.write_read(Self::BASE_ADDR as u16, &[register], &mut b).map_err(|e| e.to_string())?;
        Ok(b[0])
    }
}