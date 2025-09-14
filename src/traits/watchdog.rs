use clap::Command;

use crate::cli::args;
use crate::traits::Card;
use crate::{Error, Result};

pub trait WatchdogInfo {
    const RESET: u8;
    const INTERVAL_SET: u8;
    const INTERVAL_GET: u8;
    const INIT_INTERVAL_SET: u8;
    const INIT_INTERVAL_GET: u8;
    const RESET_COUNT: u8;
    const CLEAR_RESET_COUNT: u8;
    const POWER_OFF_INTERVAL_SET: u8;
    const POWER_OFF_INTERVAL_GET: u8;

    const RESET_SIGNATURE: u8 = 0xca;
    const RESET_COUNT_SIGNATURE: u8 = 0xbe;

    const INTERVAL_SIZE: u8 = 2;
    const MAX_INTERVAL: u64 = 1 << (8 * Self::INTERVAL_SIZE);

    const INIT_INTERVAL_SIZE: u8 = 2;
    const MAX_INIT_INTERVAL: u64 = 1 << (8 * Self::INIT_INTERVAL_SIZE);

    const OFF_INTERVAL_SIZE: u8 = 4;
    const MAX_OFF_INTERVAL: u64 = 48 * 60 * 60; // 48 hours

    const RESET_COUNT_SIZE: u8 = 2;
}

pub trait Watchdog {
    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()>;
    fn watchdog_cmd(&self) -> Command;

    fn reload_cmd(&self) -> Command;
    fn reload(&self) -> Result<()>;

    fn get_interval_cmd(&self) -> Command;
    fn get_interval(&self) -> Result<u32>;

    fn set_interval_cmd(&self) -> Command;
    fn set_interval(&self, state: u32) -> Result<()>;

    fn get_init_interval_cmd(&self) -> Command;
    fn get_init_interval(&self) -> Result<u32>;

    fn set_init_interval_cmd(&self) -> Command;
    fn set_init_interval(&self, state: u32) -> Result<()>;

    fn get_off_interval_cmd(&self) -> Command;
    fn get_off_interval(&self) -> Result<u32>;

    fn set_off_interval_cmd(&self) -> Command;
    fn set_off_interval(&self, state: u32) -> Result<()>;

    fn get_reset_count_cmd(&self) -> Command;
    fn get_reset_count(&self) -> Result<u32>;

    fn clear_reset_count_cmd(&self) -> Command;
    fn clear_reset_count(&self) -> Result<()>;
}

impl<T> Watchdog for T
where
    T: WatchdogInfo + Card,
{
    fn watchdog_cmd(&self) -> Command {
        Command::new("watchdog")
            .about("Watchdog related commands")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(self.reload_cmd())
            .subcommand(self.get_interval_cmd())
            .subcommand(self.set_interval_cmd())
            .subcommand(self.get_init_interval_cmd())
            .subcommand(self.set_init_interval_cmd())
            .subcommand(self.get_off_interval_cmd())
            .subcommand(self.set_off_interval_cmd())
            .subcommand(self.get_reset_count_cmd())
            .subcommand(self.clear_reset_count_cmd())
    }

    fn reload_cmd(&self) -> Command {
        Command::new("reload").about("Reload the watchdog timer")
    }
    fn reload(&self) -> Result<()> {
        self.write_u8(Self::RESET, Self::RESET_SIGNATURE)?;
        Ok(())
    }

    fn get_interval_cmd(&self) -> Command {
        Command::new("get-interval").about("Get watchdog timer interval state for specified channel")
    }
    fn get_interval(&self) -> Result<u32> {
        let val = self.read_u_n(Self::INTERVAL_GET, Self::INTERVAL_SIZE)?;
        Ok(val)
    }

    fn set_interval_cmd(&self) -> Command {
        Command::new("set-interval")
            .about("Set watchdog timer interval")
            .arg(args::interval(Self::MAX_INTERVAL))
    }
    fn set_interval(&self, interval: u32) -> Result<()> {
        self.write_u_n(Self::INTERVAL_SET, Self::INTERVAL_SIZE, interval)?;
        Ok(())
    }

    fn get_init_interval_cmd(&self) -> Command {
        Command::new("get-init-interval").about("Get watchdog timer initial interval state")
    }
    fn get_init_interval(&self) -> Result<u32> {
        let val = self.read_u_n(Self::INIT_INTERVAL_GET, Self::INIT_INTERVAL_SIZE)?;
        Ok(val)
    }

    fn set_init_interval_cmd(&self) -> Command {
        Command::new("set-init-interval")
            .about("Set watchdog timer initial interval")
            .arg(args::interval(Self::MAX_INIT_INTERVAL))
    }
    fn set_init_interval(&self, interval: u32) -> Result<()> {
        self.write_u_n(Self::INIT_INTERVAL_SET, Self::INIT_INTERVAL_SIZE, interval)?;
        Ok(())
    }

    fn get_off_interval_cmd(&self) -> Command {
        Command::new("get-off-interval").about("Get watchdog timer power-off interval state")
    }
    fn get_off_interval(&self) -> Result<u32> {
        let val = self.read_u_n(Self::POWER_OFF_INTERVAL_GET, Self::OFF_INTERVAL_SIZE)?;
        Ok(val)
    }

    fn set_off_interval_cmd(&self) -> Command {
        Command::new("set-off-interval")
            .about("Set watchdog timer power-off interval")
            .arg(args::interval(Self::MAX_OFF_INTERVAL))
    }
    fn set_off_interval(&self, interval: u32) -> Result<()> {
        self.write_u_n(Self::POWER_OFF_INTERVAL_SET, Self::OFF_INTERVAL_SIZE, interval)?;
        Ok(())
    }

    fn get_reset_count_cmd(&self) -> Command {
        Command::new("get-reset-count").about("Get watchdog timer reset count")
    }
    fn get_reset_count(&self) -> Result<u32> {
        let val = self.read_u_n(Self::RESET_COUNT, Self::RESET_COUNT_SIZE)?;
        Ok(val)
    }

    fn clear_reset_count_cmd(&self) -> Command {
        Command::new("clear-reset-count").about("Clear watchdog timer reset count")
    }
    fn clear_reset_count(&self) -> Result<()> {
        self.write_u8(Self::CLEAR_RESET_COUNT, Self::RESET_COUNT_SIGNATURE)?;
        Ok(())
    }

    fn handle_cmd(&self, matches: &clap::ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("reload", _)) => {
                self.reload()?;
                println!("Watchdog timer reloaded");
                Ok(())
            }
            Some(("get-interval", _)) => {
                let interval = self.get_interval()?;
                println!("Watchdog interval: {} seconds", interval);
                Ok(())
            }
            Some(("set-interval", sub_m)) => {
                let interval: u64 = *sub_m.get_one::<u64>("interval").unwrap();
                self.set_interval(interval as u32)?;
                println!("Watchdog interval set to {} seconds", interval);
                Ok(())
            }
            Some(("get-init-interval", _)) => {
                let interval = self.get_init_interval()?;
                println!("Watchdog initial interval: {} seconds", interval);
                Ok(())
            }
            Some(("set-init-interval", sub_m)) => {
                let interval: u64 = *sub_m.get_one::<u64>("interval").unwrap();
                self.set_init_interval(interval as u32)?;
                println!("Watchdog initial interval set to {} seconds", interval);
                Ok(())
            }
            Some(("get-off-interval", _)) => {
                let interval = self.get_off_interval()?;
                println!("Watchdog power-off interval: {} seconds", interval);
                Ok(())
            }
            Some(("set-off-interval", sub_m)) => {
                let interval: u64 = *sub_m.get_one::<u64>("interval").unwrap();
                self.set_off_interval(interval as u32)?;
                println!("Watchdog power-off interval set to {} seconds", interval);
                Ok(())
            }
            Some(("get-reset-count", _)) => {
                let count = self.get_reset_count()?;
                println!("Watchdog reset count: {}", count);
                Ok(())
            }
            Some(("clear-reset-count", _)) => {
                self.clear_reset_count()?;
                println!("Watchdog reset count cleared");
                Ok(())
            }
            _ => Err(Error::UnknownCommand {
                command: "watchdog".to_string(),
            }),
        }
    }
}
