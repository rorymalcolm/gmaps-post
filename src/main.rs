use std::fs;

use serde::{Deserialize, Serialize};

const FILE_PATH: &str = "/Users/rorymalcolm/Documents/gmaps-post/Records.json";

#[derive(Serialize, Deserialize, Debug)]
struct GmapsDataWrapper {
    locations: Vec<GmapsData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GmapsData {
    timestamp: String,
    #[serde(rename = "latitudeE7")]
    latitude_e7: i64,
    #[serde(rename = "longitudeE7")]
    longitude_e7: i64,
    accuracy: i64,
    device_tag: Option<i64>,
    source: Option<String>,
    #[serde(rename = "deviceTag")]
    velocity: Option<i64>,
    #[serde(rename = "platformType")]
    platform_type: Option<String>,
    #[serde(rename = "serverTimestamp")]
    server_timestamp: Option<String>,
    #[serde(rename = "deviceTimestamp")]
    device_timestamp: Option<String>,
    #[serde(rename = "batteryCharging")]
    battery_charging: Option<bool>,
    #[serde(rename = "formFactor")]
    form_factor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GmapsDataOutput {
    timestamp: String,
    latitude: f64,
    longitude: f64,
    accuracy: i64,
    device_tag: Option<i64>,
    velocity: Option<i64>,
    source: Option<String>,
    platform_type: Option<String>,
    server_timestamp: Option<String>,
    device_timestamp: Option<String>,
    battery_charging: Option<bool>,
    form_factor: Option<String>,
}

impl GmapsDataOutput {
    fn new(
        timestamp: String,
        latitude: f64,
        longitude: f64,
        accuracy: i64,
        device_tag: Option<i64>,
        source: Option<String>,
        velocity: Option<i64>,
        server_timestamp: Option<String>,
        device_timestamp: Option<String>,
        battery_charging: Option<bool>,
        form_factor: Option<String>,
    ) -> GmapsDataOutput {
        GmapsDataOutput {
            timestamp,
            latitude,
            longitude,
            accuracy,
            source,
            device_tag,
            velocity,
            platform_type: None,
            server_timestamp,
            device_timestamp,
            battery_charging,
            form_factor,
        }
    }

    fn from_gmaps_data(gmaps_data: GmapsData) -> GmapsDataOutput {
        let latitude = gmaps_data.latitude_e7 as f64 / 1e7;
        let longitude = gmaps_data.longitude_e7 as f64 / 1e7;
        let accuracy = gmaps_data.accuracy;
        let source = gmaps_data.source;
        let device_tag = gmaps_data.device_tag;
        let velocity = gmaps_data.velocity;
        let server_timestamp = gmaps_data.server_timestamp;
        let device_timestamp = gmaps_data.device_timestamp;
        let battery_charging = gmaps_data.battery_charging;
        let form_factor = gmaps_data.form_factor;
        GmapsDataOutput::new(
            gmaps_data.timestamp,
            latitude,
            longitude,
            accuracy,
            device_tag,
            source,
            velocity,
            server_timestamp,
            device_timestamp,
            battery_charging,
            form_factor,
        )
    }
}

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let data: GmapsDataWrapper =
        serde_json::from_str(&contents).expect("Should have been able to parse the json");
    let data_output = data
        .locations
        .iter()
        .map(|x| GmapsDataOutput::from_gmaps_data(x.clone()))
        .collect::<Vec<GmapsDataOutput>>();
    let mut file = fs::File::create("output.json").expect("Unable to create file");
    serde_json::to_writer(&mut file, &data_output).expect("Unable to write json to file");
}
