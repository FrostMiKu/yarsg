use clap::{App, AppSettings, Arg, SubCommand, crate_authors, crate_description, crate_version};
pub use super::config;

pub mod new;

pub fn build_cli() -> App<'static, 'static> {

    App::new("yarsg")
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .arg(
        Arg::with_name("root")
            .short("r")
            .long("root")
            .takes_value(true)
            .default_value(".")
            .help("Directory to use as root of project")
    )
    .arg(
        Arg::with_name("config")
            .short("c")
            .long("config")
            .takes_value(true)
            .help("Path to a config file other than config.toml in the root of project")
    )
    .subcommands(vec![
        SubCommand::with_name("new")
            .about("Create a new web site")
            .arg(
                Arg::with_name("name")
                    .default_value("yars")
                    .help("Name of the site. Will create a new directory with that name in the current directory"),
            ),
        SubCommand::with_name("build")
            .about("Build site")
            .args(&[
                Arg::with_name("name")
                    .default_value("yars")
                    .help("build site"),
            ]),
    ])
}