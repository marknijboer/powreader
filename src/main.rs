mod interpret;
mod util;
mod output;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
extern crate serial;
extern crate clap;
extern crate regex;
extern crate influx_db_client;
extern crate serde_json;
extern crate simple_error;
extern crate chrono;

use std::io;
use std::time::Duration;
use std::io::BufRead;
use clap::{ArgMatches, App, Arg};
use serial::prelude::*;

fn main() {
    let matches = load_cli_flags();
    let serial_device = matches.value_of("SERIALPORT").unwrap();
    let mut port = serial::open(serial_device).unwrap();

    interact(&mut port, &matches).unwrap();
}

// Load all flags and parse the arguments
fn load_cli_flags() -> ArgMatches<'static> {
    return App::new("Powreader")
    .version("1.0")
    .author("Mark N. <mark@nijboer.email>")
    .about("Powreader interprets the output from the digital power meter and outputs it in JSON or pushes it to an InfluxDB.")
    .arg(Arg::with_name("SERIALPORT")
        .required(true)
        .index(1)
        .help("Sets the serial port to use"))
    .arg(Arg::with_name("influxdb")
        .short("i")
        .long("influxdb")
        .takes_value(true)
        .help("Sets the full host and port to the influxdb. Data is transfered over UDP. This argument is given in the form of HOST:PORT. If this argument is not set, data will be printed as a JSON stream."))
    .get_matches();
}

fn interact<T: SerialPort>(port: &mut T, matches: &ArgMatches) -> io::Result<()> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200).expect("Cannot set baudrate");
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }).expect("Cannot reconfigure");

    // Data rate is every 1000 milliseconds but we build in an extra 20% to 
    // cancel out any accidential timeouts.
    port.set_timeout(Duration::from_millis(1200)).expect("Cannot set timeout");

    let mut reader = io::BufReader::new(port);
    let mut line = String::new();
    let mut buffer: Vec<String> = Vec::new();
    loop {
        // Make sure that we always start with an empty line buffer.
        line.truncate(0);

        // E.g. a timeout error will occur here.
        if let Err(e) = reader.read_line(&mut line) {
            eprintln!("Could not read line: {}", e);
            continue;
        }

        // A line that starts with the exclamation mark signals the last data
        // line. That means that we must have a buffer that is complete at this
        // point.
        if !line.starts_with('!') {
            buffer.push(line.clone());
            continue;
        }
        
        // Interpret the message to a struct and handle the output according to
        // the command line arguments.
        match interpret::message(&buffer) {
            Ok(meter_output) => {
                output::handle(&meter_output, matches);
            }, 
            Err(e) => {
                util::print_error(e, &buffer);
            }
        }

        // At no point there will be more than one data message in the buffer.
        // Once we interpreted it, it can be emptied so we can capture the next
        // message.
        buffer.truncate(0);
    }
}