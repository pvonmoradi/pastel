#[macro_use]
extern crate clap;

use clap::{App as ClapApp, AppSettings, Arg, SubCommand};
use ansi_term::{Style, Colour};

use palette::{Srgb};

#[derive(Debug, PartialEq)]
enum PastelError {
    ColorParseError,
}

impl PastelError {
    fn message(&self) -> &str {
        match self {
            PastelError::ColorParseError => "could not parse color",
        }
    }
}

type Result<T> = std::result::Result<T, PastelError>;

type ExitCode = i32;

fn parse_color(color: &str) -> Srgb {
    Srgb::new(0.2, 0.2, 0.9)
}

fn run() -> Result<ExitCode> {
    let app = ClapApp::new(crate_name!())
        .version(crate_version!())
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::InferSubcommands)
        .global_setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .max_term_width(100)
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("show")
                .about("Show the given color on the terminal")
                .arg(
                    Arg::with_name("color")
                        .help("Color to show")
                        .required(true)
                ),
        );

    let global_matches = app.get_matches();

    if let Some(matches) = global_matches.subcommand_matches("show") {
        let color = parse_color(matches.value_of("color").unwrap());

        let terminal_color = Colour::RGB(
            (255. * color.red) as u8,
            (255. * color.blue) as u8,
            (255. * color.green) as u8);
        let style = Style::new().on(terminal_color);

        println!();
        for _ in 0..8 {
            println!("    {}", style.paint("                "));
        }
        println!();
    } else {
        unreachable!("Unknown subcommand");
    }

    Ok(0)
}

fn main() {
    let result = run();
    match result {
        Err(err) => {
            eprintln!("Error: {}", err.message());
            std::process::exit(1);
        }
        Ok(exit_code) => {
            std::process::exit(exit_code);
        }
    }
}
