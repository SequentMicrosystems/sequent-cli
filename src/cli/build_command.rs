use crate::{Error, Result};

use clap::ArgAction;
use clap::{Arg, ArgMatches, Command};

use crate::cards::MultiIo;
use crate::traits::{Card, Opto, Relay};

pub trait Capabilities {
    fn as_card(&self) -> Result<&dyn Card> {
        Err(Error::UnsupportedCapability { capability: "Card" })
    }
    fn as_opto(&self) -> Result<&dyn Opto> {
        Err(Error::UnsupportedCapability { capability: "Opto" })
    }
    fn as_relay(&self) -> Result<&dyn crate::traits::Relay> {
        Err(Error::UnsupportedCapability { capability: "Relay" })
    }
    fn as_led(&self) -> Result<&dyn crate::traits::Led> {
        Err(Error::UnsupportedCapability { capability: "Led" })
    }
    fn as_watchdog(&self) -> Result<&dyn crate::traits::Watchdog> {
        Err(Error::UnsupportedCapability { capability: "Watchdog" })
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

pub fn build_command(dev: &dyn Capabilities) -> Command {
    let card = dev.as_card().expect("Device must implement Card trait");

    let mut cmd = Command::new(card.program_name())
        .version(card.version())
        .about(format!("{} command line interface", card.card_name()))
        .arg(
            Arg::new("info")
                .short('i')
                .long("info")
                .help("Show device information")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("stack-level")
                .short('s')
                .long("stack")
                .help("Set the stack level of the target device")
                .value_parser(clap::value_parser!(u8).range(0..card.max_stack_level() as i64))
                .default_value("0")
                .global(true),
        );

    if let Ok(opto) = dev.as_opto() {
        cmd = cmd.subcommand(opto.opto_cmd());
    }

    if let Ok(relay) = dev.as_relay() {
        // Flat commands
        cmd = cmd.subcommand(relay.get_relay_cmd()).subcommand(relay.set_relay_cmd());
    }

    if let Ok(led) = dev.as_led() {
        cmd = cmd.subcommand(led.get_led_cmd()).subcommand(led.set_led_cmd());
    }

    if let Ok(watchdog) = dev.as_watchdog() {
        // Nested commands
        cmd = cmd.subcommand(watchdog.watchdog_cmd());
    }

    cmd
}

pub fn run_command(dev: &impl Capabilities, matches: clap::ArgMatches) -> Result<()> {
    if let Some(stack_level) = matches.get_one::<u8>("stack-level") {
        println!("STACK LEVEL: {}", stack_level);
    }
    match matches.subcommand() {
        Some(("opto", sub_m)) => {
            let opto = dev.as_opto()?;
            opto.handle_cmd(sub_m)
        }
        Some(("get-relay", sub_m)) => {
            let relay = dev.as_relay()?;
            let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
            let state = relay.get_relay(channel)?;
            println!("Relay channel {} state: {}", channel, state);
            Ok(())
        }
        Some(("set-relay", sub_m)) => {
            let relay = dev.as_relay()?;
            let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
            let state: bool = *sub_m.get_one::<bool>("state").unwrap();
            relay.set_relay(channel, state)?;
            println!("Relay channel {} set to {}", channel, state);
            Ok(())
        }
        Some(("get-led", sub_m)) => {
            let led = dev.as_led()?;
            let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
            let state = led.get_led(channel)?;
            println!("LED channel {} state: {}", channel, state);
            Ok(())
        }
        Some(("set-led", sub_m)) => {
            let led = dev.as_led()?;
            let channel: u8 = *sub_m.get_one::<u8>("channel").unwrap();
            let state: bool = *sub_m.get_one::<bool>("state").unwrap();
            led.set_led(channel, state)?;
            println!("LED channel {} set to {}", channel, state);
            Ok(())
        }
        Some(("watchdog", sub_m)) => {
            let watchdog = dev.as_watchdog()?;
            watchdog.handle_cmd(sub_m)
        }
        None => {
            if matches.get_flag("info") {
                let card = dev.as_card()?;
                println!("Program: {}", card.program_name());
                println!("Version: {}", card.version());
                println!("Card:    {}", card.card_name());
                println!("I2C Addr: 0x{:02X}", card.base_addr());
                // Add more info as needed
                Ok(())
            } else {
                Err(Error::NoCommand())
            }
        }
        Some((unimplemented_command, _)) => {
            panic!("Command '{}' is not implemented for this device", unimplemented_command)
        }
    }
}
