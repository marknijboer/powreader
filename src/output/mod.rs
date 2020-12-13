mod influx;

use crate::interpret::MeterOutput;
use clap::ArgMatches;

pub fn handle(meter_output: &MeterOutput, matches: &ArgMatches) {

    // If the influxdb parameter is given we send our output to the influx output
    // handler.
    if matches.value_of("influxdb").is_some() {
        influx::send_stats(matches.value_of("influxdb").unwrap(), &meter_output);
    } else {
        // If no output handler matches, we just print the output as JSON to
        // stdout.
        println!("{}", meter_output.to_json());
    }
}