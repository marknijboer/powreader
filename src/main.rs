mod interpret;
mod util;
mod influx;

extern crate serial;
extern crate clap;

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate influx_db_client;

#[macro_use]
extern crate serde;
extern crate serde_json;

use std::io;
use std::time::Duration;
use std::io::BufRead;
use clap::{ArgMatches, App, Arg};
use serial::prelude::*;

fn main() {
    let matches = load_flags();
    let serial_device = matches.value_of("SERIALPORT").unwrap();
    let mut port = serial::open(serial_device).unwrap();

    interact(&mut port, &matches).unwrap();
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

    port.set_timeout(Duration::from_millis(1000)).expect("Cannot set timeout");

    let mut reader = io::BufReader::new(port);
    let mut line = String::new();
    let mut buffer: Vec<String> = Vec::new();
    loop {
        reader.read_line(&mut line)?;
        //print!("{}", line);
        if line.starts_with('!') {
            if let Ok(output) = interpret::message(&buffer) {
                if matches.value_of("influxdb").is_none() {
                    println!("{}", output.to_json());
                } else {
                    // Send via influxdb
                    influx::send_stats(matches.value_of("influxdb").unwrap(), &output)
                }
            }
        } else {
            buffer.push(line.clone());
        }
        
        line.truncate(0);
    }
}

/// Load all flags and parse the arguments
fn load_flags() -> ArgMatches<'static> {
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