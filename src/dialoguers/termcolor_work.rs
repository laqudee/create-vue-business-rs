use crate::utils::get_command::get;
use crate::ConfiguresSelected;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn work(_project_name: &String, _configures_selected: &ConfiguresSelected) -> io::Result<()> {
    let package_manager = "pnpm";

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Set the color and style of the output
    let mut style = ColorSpec::new();
    style.set_fg(Some(Color::Cyan)).set_bold(true);
    stdout.set_color(&style)?;

    writeln!(&mut stdout, "Done. Now run:")?;

    style.set_fg(Some(Color::Blue)).set_bold(true);
    stdout.set_color(&style)?;

    writeln!(
        &mut stdout,
        "Please use pnpm as the package management tool for the workspace project"
    )?;

    style.set_fg(Some(Color::Green)).set_bold(true);
    stdout.set_color(&style)?;

    writeln!(&mut stdout, "  cd ../")?;
    writeln!(&mut stdout, "  {}", get(package_manager, "install", None))?;
    writeln!(&mut stdout, "  {}", get(package_manager, "dev", None))?;

    style.set_fg(Some(Color::White)).set_bold(false);
    stdout.set_color(&style)?;
    writeln!(&mut stdout, "")?;

    Ok(())
}
