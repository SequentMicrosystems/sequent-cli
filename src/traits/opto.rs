use crate::traits::Card;

pub trait OptoAddrs {
    const OPTO: u8;
    const RISING:  u8;
    const FALLING: u8;
    const ENCODER_ENABLE: u8;
    const COUNTER_RESET:    u8;
    const ENCODER_COUNTER_RESET: u8;
    const EDGE_COUNT: u8;
    const ENCODER_COUNT: u8;

    // board parameters
    const OPTO_CH_NO: usize;
    const COUNTER_SIZE: usize;
}

pub trait Opto: OptoAddrs + Card  {
    fn read_opto(&self, channel: usize) -> Result<bool, String> {
        let val = self.read_u8(Self::OPTO + channel as u8)?;
        Ok(val & (1 << channel) != 0)
    }
}