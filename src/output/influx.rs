use crate::interpret::MeterOutput;
use influx_db_client::{UdpClient, Point, Value, point};
use std::net::SocketAddr;

pub fn send_stats(host: &str, data: &MeterOutput) {
    let server_opt = host
        .parse::<SocketAddr>();

    if server_opt.is_err() {
        eprintln!("Could not parse InfluxDB address");
        return;
    }

    let server = server_opt.unwrap();

    let mut udp = UdpClient::new(server);
    udp.add_host(server);

    let point = point!("power")
    .add_field("actual_delivered_power", Value::Float(data.actual_delivered_power))
    .add_field("actual_returned_power", Value::Float(data.actual_returned_power))
    .add_field("power_failures", Value::Integer(data.power_failures as i64))
    .add_field("long_power_failures", Value::Integer(data.long_power_failures as i64))
    .add_field("power_delivered_tariff1", Value::Float(data.power_delivered_tariff1))
    .add_field("power_delivered_tariff2", Value::Float(data.power_delivered_tariff2))
    .add_field("power_returned_tariff1", Value::Float(data.power_returned_tariff1))
    .add_field("power_returned_tariff2", Value::Float(data.power_returned_tariff2))
    .add_field("power_tariff_indicator", Value::Integer(data.tariff_indicator as i64))
    .add_field("power_equipment_id", Value::String(data.power_equipment_id.clone()));

    // Drawing 1 MW in once seems like an invalid value. We will skip that.
    if data.actual_delivered_power > 1000.0 {
        return;
    }

    if let Err(e) = udp.write_point(point) {
        eprintln!("Could not write data point to InfluxDB: {}", e);
    }
}