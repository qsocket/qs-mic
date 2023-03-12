use clap::{arg, App};
use colored::Colorize;
use std::env;

pub struct CommandParams {
    pub device: String,
    pub jack: String,
    pub secret: String,
    pub output: String,
    pub rec_duration: u8,
    pub probe: i32,
    pub play: bool,
    pub listen: bool,
    pub generate: bool,
    pub verify_cert: bool,
    pub verbose: bool,
    pub quiet: bool,
}

pub fn parse_options() -> Result<CommandParams, anyhow::Error> {
    let app = App::new("qs-mic")
        .version("1.0")
        .author("Ege BALCI. <egebalci@pm.me>")
        .about("Qsocket microphone utility.")
        .arg(arg!([DEVICE] "The audio device to use").default_value("default"))
        .arg(arg!(-j --jack "Use the JACK host"))
        .arg(arg!(-o --out [INPUT] "File name for received recording."))
        .arg(arg!(-s --secret [INPUT] "Secret. (e.g. password)."))
        .arg(arg!(-l --listen "Run in server mode. [default: client]"))
        .arg(arg!(-g --generate  "Generate a random secret."))
        .arg(arg!(-d --duration [INPUT] "Microphone record duration.").default_value("10"))
        .arg(arg!(-t --probe [INPUT] "Probe interval for QSRN.").default_value("5"))
        .arg(arg!(--pin  "Enable certificate fingerprint verification on TLS connections."))
        .arg(arg!(-q --quiet "Disable output."))
        .arg(arg!(-v --verbose "Verbose output."))
        .arg(arg!(--play "Play the recording while receiving."));
    //.arg(arg!(-T --tor "Use TOR."))
    let matches: clap::ArgMatches;
    if let Ok(env_args) = env::var("QS_ARGS") {
        let mut args = vec!["qs-mic"];
        args.append(env_args.split_whitespace().collect::<Vec<&str>>().as_mut());
        matches = app.get_matches_from(args);
    } else {
        matches = app.get_matches();
    }

    let empty = &String::new();
    // Create the command parameters struct
    let device: &String = matches.get_one("DEVICE").unwrap();
    let jack: &String = matches.get_one("jack").unwrap_or(empty);
    let secret: &String = matches.get_one("secret").unwrap_or(empty);
    let probe: &String = matches.get_one("probe").unwrap();
    let duration: &String = matches.get_one("duration").unwrap();
    let output: &String = matches.get_one("out").unwrap_or(empty);

    let opts = CommandParams {
        device: device.to_string(),
        jack: jack.to_string(),
        secret: secret.to_string(),
        rec_duration: duration.parse::<u8>().unwrap(),
        output: output.to_string(),
        probe: probe.parse::<i32>().unwrap(),
        listen: matches.is_present("listen"),
        generate: matches.is_present("generate"),
        verify_cert: matches.is_present("pin"),
        play: matches.is_present("play"),
        verbose: matches.is_present("verbose"),
        quiet: matches.is_present("quiet"),
    };

    Ok(opts)
}

pub fn summarize_options(opts: &CommandParams) {
    let mut mode = "client";
    if opts.listen {
        mode = "server";
    }

    println!(
        "{} {}",
        "[#]".yellow().bold(),
        ".:: Qsocket Mic ::.".blue().bold()
    );
    println!("{} Secret: {}", "├──>".yellow(), opts.secret.red());
    println!("{} Mode: {}", "├──>".yellow(), mode);
    if !opts.listen {
        println!("{} Rec. Duration: {}", "├──>".yellow(), opts.rec_duration);
    }
    if opts.listen {
        println!("{} Probe Interval: {}", "└──>".yellow(), opts.probe);
    } else {
        println!("{} Probe Duration: {}", "└──>".yellow(), opts.probe);
    }
    println!(" ");
}
