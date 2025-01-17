
use clap::{command, Arg, ArgAction, ArgMatches, Command};

use intern::*;

fn main() {
    let matches = cli().get_matches();
    from_matches(&matches);
    intern::run();
}

fn cli() -> Command {
    command!().args([
        Arg::new("tr_zeroes")
            .required(true)
            .short('N')
            .id("N")
            .action(ArgAction::Set)
            .value_name("number")
            .help("Specify number of trailing zeroes"),
        Arg::new("results")
            .required(true)
            .short('F')
            .id("F")
            .action(ArgAction::Set)
            .value_name("number")
            .help("Specify amount of results"),
    ])
}

fn from_matches(matches: &ArgMatches) {
    let aor = matches.get_one::<String>("F").unwrap();
    let tz = matches.get_one::<String>("N").unwrap();

    let aor: usize = aor
        .parse()
        .expect(&format!("\nExpected number, got \"{aor}\"\n\n"));
    let tz: usize = tz
        .parse()
        .expect(&format!("\nExpected number, got \"{tz}\"\n\n"));

    set_statics(aor, tz)
}
