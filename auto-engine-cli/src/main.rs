use crate::cmd::{Cli, Commands, ConfigType};
use crate::converter::keymousego::{Converter, ConverterFrom};
use clap::Parser;
use std::path::PathBuf;
use crate::converter::quickinput;

mod cmd;
mod converter;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Convert { config_type } => match config_type {
            ConfigType::KeyMouseGo { config } => {
                let path = PathBuf::from(config);
                let result = Converter::new(&path)
                    .convert(ConverterFrom::KeyMouseGo)
                    .unwrap();

                let content = serde_yaml::to_string(&result).unwrap();

                println!("{}", content);
            },
            ConfigType::QuickInput { config } => {
                let path = PathBuf::from(config);
                let result = quickinput::Converter::new(&path)
                    .convert().unwrap();
                let content = serde_yaml::to_string(&result).unwrap();
                println!("{}", content);
            }
        },
    }
}
