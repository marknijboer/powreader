use crate::interpret::MeterOutput;
use influx_db_client::{UdpClient, Point, Value, point};
use std::net::SocketAddr;

pub fn send_stats(host: &str, data: &MeterOutput) {
    let server_opt= host
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
    .add_field("actual_returned_power", Value::Float(data.actual_delivered_power))
    .add_field("power_failures", Value::Float(data.actual_delivered_power))
    .add_field("long_power_failures", Value::Float(data.actual_delivered_power))
    .add_field("power_delivered_tariff1", Value::Float(data.actual_delivered_power))
    .add_field("power_delivered_tariff2", Value::Float(data.actual_delivered_power))
    .add_field("power_returned_tariff1", Value::Float(data.actual_delivered_power))
    .add_field("power_returned_tariff2", Value::Float(data.actual_delivered_power))
    .add_field("power_tariff_indicator", Value::Float(data.actual_delivered_power))
    .add_field("power_equipment_id", Value::String(data.power_equipment_id.clone()));

    udp.write_point(point).unwrap();
}