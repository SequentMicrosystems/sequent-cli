use crate::{Error, Result};

use clap::ArgAction;
use clap::{Arg, ArgMatches, Command};

use crate::cards::MultiIo;
use crate::traits::CardInfo;
use crate::traits::{Card, Opto, Relay};

pub trait Capabilities {
    fn as_opto(&self) -> Result<&dyn Opto> {
        Err(Error::UnsupportedCapability { capability: "Opto" })
    }
    fn as_relay(&self) -> Result<&dyn crate::traits::Relay> {
        Err(Error::UnsupportedCapability { capability: "Relay" })
    }
    fn as_led(&self) -> Result<&dyn crate::traits::Led> {
        Err(Error::UnsupportedCapability { capability: "Led" })
    }
    fn as_ledmode(&self) -> Result<&dyn crate::traits::LedMode> {
        Err(Error::UnsupportedCapability { capability: "LedMode" })
    }
    fn as_watchdog(&self) -> Result<&dyn crate::traits::Watchdog> {
        Err(Error::UnsupportedCapability { capability: "Watchdog" })
    }
    fn as_rtd(&self) -> Result<&dyn crate::traits::Rtd> {
        Err(Error::UnsupportedCapability { capability: "Rtd" })
    }
    fn as_rtdcalib(&self) -> Result<&dyn crate::traits::RtdCalib> {
        Err(Error::UnsupportedCapability { capability: "RtdCalib" })
    }
    fn as_calib(&self) -> Result<&dyn crate::traits::Calib> {
        Err(Error::UnsupportedCapability { capability: "Calib" })
    }
    fn as_motor(&self) -> Result<&dyn crate::traits::Motor> {
        Err(Error::UnsupportedCapability { capability: "Motor" })
    }
    fn as_servo(&self) -> Result<&dyn crate::traits::Servo> {
        Err(Error::UnsupportedCapability { capability: "Servo" })
    }
    fn as_outputv0_10(&self) -> Result<&dyn crate::traits::OutputV0_10> {
        Err(Error::UnsupportedCapability {
            capability: "OutputV0_10",
        })
    }
}

/*
// Macro that auto implements Capabilities
impl Capabilities for MultiIo {
    fn as_opto(&self) -> Option<&dyn Opto> { Some(self) }
    fn as_card(&self) -> Option<&dyn traits::Card> { Some(self) }
}
impl_capabilities!(MultiIo; Opto, Card);
*/
#[macro_export]
macro_rules! impl_capabilities {
    ($ty:ty; $( $feat:ident ),+ $(,)?) => {
        impl $crate::Capabilities for $ty {
            $(
                paste::paste! {
                    #[inline]
                    fn [<as_ $feat:lower>](&self) -> crate::Result<&dyn $feat> {
                        Ok(self)
                    }
                }
            )*
        }
    };
}

use clap::builder::BoolishValueParser;

pub fn build_command<T>(dev: &T) -> Command
where
    T: Capabilities + Card,
{
    let mut cmd = Command::new(dev.program_name())
        .version(dev.version())
        .about(format!("{} HAT command line interface", dev.card_name()))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("stack-level")
                /*
                .short('s')
                .long("stack")
                */
                .value_name("STACK_LEVEL")
                .help("Set the stack level of the target device")
                .value_parser(clap::value_parser!(u8).range(0..=dev.max_stack_level() as i64))
                .default_value("0")
                .global(true),
        );
    if let Ok(opto) = dev.as_opto() {
        cmd = cmd.subcommand(opto.opto_cmd());
    }

    if let Ok(relay) = dev.as_relay() {
        cmd = cmd.subcommand(relay.relay_cmd());
        // Flat commands
        //cmd = cmd.subcommand(relay.get_relay_cmd()).subcommand(relay.set_relay_cmd());
    }

    if let Ok(led) = dev.as_led() {
        let mut led_cmd = led.led_cmd();
        // Extend led_cmd with similar capabilities
        if let Ok(led_mode) = dev.as_ledmode() {
            led_cmd = led_cmd.subcommands([led_mode.get_led_mode_cmd(), led_mode.set_led_mode_cmd()]);
        }
        cmd = cmd.subcommand(led_cmd);
    }

    if let Ok(rtd) = dev.as_rtd() {
        let mut rtd_cmd = rtd.rtd_cmd();
        if let Ok(rtd_calib) = dev.as_rtdcalib() {
            rtd_cmd = rtd_cmd.subcommands([rtd_calib.calib_rtd_cmd(), rtd_calib.reset_calib_rtd_cmd()]);
        }
        cmd = cmd.subcommand(rtd_cmd);
    }

    if let Ok(watchdog) = dev.as_watchdog() {
        cmd = cmd.subcommand(watchdog.watchdog_cmd());
    }

    if let Ok(calib) = dev.as_calib() {
        cmd = cmd.subcommand(calib.calib_status_cmd());
    }

    if let Ok(motor) = dev.as_motor() {
        cmd = cmd.subcommand(motor.motor_cmd());
    }

    if let Ok(servo) = dev.as_servo() {
        cmd = cmd.subcommand(servo.servo_cmd());
    }

    if let Ok(outputv0_10) = dev.as_outputv0_10() {
        cmd = cmd.subcommand(outputv0_10.output_cmd());
    }

    cmd = cmd.subcommand(dev.info_cmd());

    cmd
}

pub fn run_command<T>(dev: &mut T, matches: clap::ArgMatches) -> Result<()>
where
    T: Capabilities + Card + CardInfo,
{
    if let Some(stack_level) = matches.get_one::<u8>("stack-level") {
        // TODO: Change this, integrate it better in the answer
        println!("Stack level: {}", stack_level);
        dev.set_stack_level(*stack_level)
    }
    match matches.subcommand() {
        Some(("led", sub_m)) => {
            let led = dev.as_led()?;
            led.handle_cmd(sub_m)
            // TODO: Nest with ledmode
        }
        Some(("relay", sub_m)) => {
            let relay = dev.as_relay()?;
            relay.handle_cmd(sub_m)
        }
        Some(("opto", sub_m)) => {
            let opto = dev.as_opto()?;
            opto.handle_cmd(sub_m)
        }
        Some(("watchdog", sub_m)) => {
            let watchdog = dev.as_watchdog()?;
            watchdog.handle_cmd(sub_m)
        }
        Some(("rtd", sub_m)) => {
            let rtd = dev.as_rtd()?;
            rtd.handle_cmd(sub_m)
            // TODO: Nest with rtd calib
        }
        // TODO: Add "calib"
        Some(("motor", sub_m)) => {
            let motor = dev.as_motor()?;
            motor.handle_cmd(sub_m)
        }
        Some(("servo", sub_m)) => {
            let servo = dev.as_servo()?;
            servo.handle_cmd(sub_m)
        }
        // TODO: Make a list where all the subcommands are added, so you don't hardcode this "uout" in here
        Some(("uout", sub_m)) => {
            let outputv0_10 = dev.as_outputv0_10()?;
            outputv0_10.handle_cmd(sub_m)
        }
        Some(("info", _)) => {
            println!("Program: {}", dev.program_name());
            println!("Version: {}", dev.version());
            println!("Card:    {}", dev.card_name());
            println!("I2C Addr: 0x{:02X}", dev.base_addr());
            // Add more info as needed
            Ok(())
        }
        None => Err(Error::NoCommand()),
        Some((unimplemented_command, _)) => {
            panic!("Command '{}' is not implemented for this device", unimplemented_command)
        }
    }
}
