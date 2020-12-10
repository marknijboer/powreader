use crate::util::{ 
    parse_numeric,
    parse_timestamp,
    get_single_bracket_data,
    get_double_bracket_data,
    divide_by_thousand
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterOutput {
    pub p1_output_version: i32,
    pub timestamp: i64,
    pub power_equipment_id: String,
    pub power_delivered_tariff1: f64,
    pub power_delivered_tariff2: f64,
    pub power_returned_tariff1: f64,
    pub power_returned_tariff2: f64,
    pub tariff_indicator: i32,
    pub actual_delivered_power: f64,
    pub actual_returned_power: f64,
    pub power_failures: i32,
    pub long_power_failures: i32,
    pub device_type: i32,
    pub gas_equipment_id: Option<String>,
    pub gas_delivered_value: Option<f64>,
    pub gas_delivered_timestamp: Option<i64>,
    pub water_equipment_id: Option<String>,
    pub water_delivered_value: Option<i32>,
    pub water_delivered_timestamp: Option<i64>
}

impl MeterOutput {
    pub fn new() -> Self {
        Self {
            p1_output_version: 0,
            timestamp: 0,
            power_equipment_id: String::new(),
            power_delivered_tariff1: 0.0,
            power_delivered_tariff2: 0.0,
            power_returned_tariff1: 0.0,
            power_returned_tariff2: 0.0,
            tariff_indicator: 0,
            actual_delivered_power: 0.0,
            actual_returned_power: 0.0,
            power_failures: 0,
            long_power_failures: 0,
            device_type: 0,
            gas_equipment_id: None,
            gas_delivered_value: None,
            gas_delivered_timestamp: None,
            water_equipment_id: None,
            water_delivered_timestamp: None,
            water_delivered_value: None,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::ser::to_string(&self).unwrap()
    }
}

pub fn message(message: &[String]) -> MeterOutput {
    let mut output = MeterOutput::new();

    for line in message {
        // Find the P1 data output version
        if line.starts_with("1-3:0.2.8") {
            let version = get_single_bracket_data(line);
            output.p1_output_version = version.parse().unwrap_or_default();
        }

        // Get the current time
        if line.starts_with("0-0:1.0.0") {
            let time = get_single_bracket_data(line);
            let unix = parse_timestamp(time);

            output.timestamp = unix;
        }

        // Get the current usage
        if line.starts_with("1-0:1.7.0") {
            let usage = get_single_bracket_data(line);
            output.actual_delivered_power = divide_by_thousand(parse_numeric(usage).as_str());
        }

        // Get the current returned
        if line.starts_with("1-0:2.7.0") {
            let usage = get_single_bracket_data(line);
            output.actual_returned_power = divide_by_thousand(parse_numeric(usage).as_str());
        }

        // Tariff 1 delivered
        if line.starts_with("1-0:1.8.1") {
            let usage = get_single_bracket_data(line);
            output.power_delivered_tariff1 = divide_by_thousand(parse_numeric(usage).as_str());
        }

         // Tariff 2 delivered
        if line.starts_with("1-0:1.8.2") {
            let usage = get_single_bracket_data(line);
            output.power_delivered_tariff2 = divide_by_thousand(parse_numeric(usage).as_str());
        }

         // Tariff 1 returned
        if line.starts_with("1-0:2.8.1") {
            let usage = get_single_bracket_data(line);
            output.power_returned_tariff1 = divide_by_thousand(parse_numeric(usage).as_str());
        }

         // Tariff 2 returned
        if line.starts_with("1-0:2.8.2") {
            let usage = get_single_bracket_data(line);
            output.power_returned_tariff2 = divide_by_thousand(parse_numeric(usage).as_str());
        }

        // Tariff indicator
        if line.starts_with("0-0:96.14.0") {
            let line = get_single_bracket_data(line);
            output.tariff_indicator = parse_numeric(line).parse().unwrap_or_default();
        }

        // Equipment ID indicator
        if line.starts_with("0-0:96.1.1") {
            let line = get_single_bracket_data(line);
            output.power_equipment_id = line.trim().to_string();
        }

        // Failures
        if line.starts_with("0-0:96.7.21") {
            let line = get_single_bracket_data(line);
            output.power_failures = line.parse().unwrap_or_default();
        }

        // Failures long
        if line.starts_with("0-0:96.7.9") {
            let line = get_single_bracket_data(line);
            output.long_power_failures = line.parse().unwrap_or_default();
        }

        // Device type
        if line.starts_with("0-1:24.1.0") {
            let line = get_single_bracket_data(line);
            output.device_type = line.parse().unwrap_or_default();
        }

        // Gas meter equipment id
        if line.starts_with("0-1:96.1.0") {
            let line = get_single_bracket_data(line);
            output.gas_equipment_id = Some(line.trim().to_string());
        }

        // Gas meter values
        if line.starts_with("0-1:24.2.1") {
            let line = get_double_bracket_data(line);
            let timestamp = line[0];
            let usage = parse_numeric(line[1]);
            output.gas_delivered_timestamp = Some(parse_timestamp(timestamp));
            let gas_volume_str = &usage[0..usage.len()-1];
            let gas_volume_double = divide_by_thousand(gas_volume_str);
            output.gas_delivered_value = Some(gas_volume_double);
        }
    }

    output
}