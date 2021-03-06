use clap::{Arg, App, ArgMatches};
use libc::{getpid};

pub fn print(log: &str) {
  println!("[hegel ({})] {}", unsafe { getpid() }, log);
}

pub fn get_arguments<'a>() -> ArgMatches<'a> {
  let matches = 
    App::new(env!("CARGO_PKG_NAME"))
      .version(env!("CARGO_PKG_VERSION"))
      .author(env!("CARGO_PKG_AUTHORS"))
      .about(env!("CARGO_PKG_DESCRIPTION"))
      .arg(Arg::with_name("process")
        .short("p")
        .long("process")
        .help("Defines a managed process")
        .takes_value(true)
        .required(true)
        .multiple(true))
      .arg(Arg::with_name("timeout")
        .short("t")
        .long("timeout")
        .help("Set the maximum time hegel should wait before killing a process in seconds, defaults to 5")
        .takes_value(true))
      .get_matches();

  matches
}

pub fn parse_arguments<'a>(matches: &'a ArgMatches) -> Vec<(&'a str, Vec<&'a str>)> {
  procs_to_pairs(matches.values_of("process").unwrap().collect())
}

fn procs_to_pairs(vals: Vec<&str>) -> Vec<(&str, Vec<&str>)> {
  vals.iter()
    .map(|&s| s.split(' ').collect())
    .map(|v: Vec<_>| (v[0], v[1..].to_owned()))
    .collect()
}
