use std::{env, fs, path::PathBuf};
use serde_json::{Result, Value};
use serde::Deserialize;

fn main() -> Result<()> {
    let arg_path = env::args().nth(1).expect("No path given");
    let as_path = PathBuf::from(arg_path);

    for entry in fs::read_dir(&as_path).expect("unable to read directory") {
        let entry = entry.expect("cannot unwrap entry");
        let path = entry.path();
        let metadata = fs::metadata(&path).expect("unable to extract metadata");

        if metadata.is_file() {
            let filename = path.file_name().expect("no file name found").to_str().unwrap();

            println!("parsing file {:?}", filename);

            let f_path = PathBuf::from(&as_path).join(filename);

            // TODO: handle errors better here
            let contents = fs::read_to_string(f_path).expect("cannot read file to string");

            //let j: Value = serde_json::from_str(&contents)?;
            let j: FlightStats = serde_json::from_str(&contents)?;
            println!("percent complete {:?}, altitude: {:?}", j.percent_flight_complete, j.altitude_value);
        }
    }

    Ok(())
}

/*

    Full output sample:

{
  "dist_remaink": 902.4000000000001,
  "headingAbbrStr": "E",
  "dtzone": "CDT",
  "lonew": "West",
  "latns": "North",
  "public_ssid": "unknown",
  "dstate": "TX",
  "gspdm": "890 kph",
  "airline_icao": "swa",
  "alt": "37 005 ft",
  "comproute": false,
  "octime": "09:46 PM",
  "orig_weather": { "orig_code": "KAUS", "success": true, "fetch_time": 1696040873, "current_conditions": { "code": "116", "moonset": "07:32 AM", "temp_f": "86", "temp_c": "30", "humidity": "57", "sunset24": "19:19", "sunset": "07:19 PM", "sunrise": "07:23 AM", "sunrise24": "07:23", "condition": "Partly cloudy" }, "forecast_conditions_2": { "code": "176", "high_f": "97", "moonset": "08:40 AM", "high_c": "36", "day_of_week": "Saturday", "low_c": "25", "low_f": "76", "condition": "Patchy rain possible", "date": "2023-09-30", "sunset": "07:18 PM", "sunrise": "07:24 AM" }, "forecast_conditions_1": { "code": "113", "high_f": "99", "moonset": "07:32 AM", "high_c": "37", "day_of_week": "Friday", "low_c": "24", "low_f": "76", "condition": "Sunny", "date": "2023-09-29", "sunset": "07:19 PM", "sunrise": "07:23 AM" }, "weather_update_cnt": 12, "forecast_conditions_3": { "code": "176", "high_f": "95", "moonset": "09:48 AM", "high_c": "35", "day_of_week": "Sunday", "low_c": "23", "low_f": "73", "condition": "Patchy rain possible", "date": "2023-10-01", "sunset": "07:17 PM", "sunrise": "07:25 AM" }, "forecast_conditions_4": { "code": "113", "high_f": "94", "moonset": "10:55 AM", "high_c": "34", "day_of_week": "Monday", "low_c": "23", "low_f": "73", "condition": "Sunny", "date": "2023-10-02", "sunset": "07:15 PM", "sunrise": "07:25 AM" }, "forecast_conditions_5": { "code": "302", "high_f": "89", "moonset": "12:01 PM", "high_c": "32", "day_of_week": "Tuesday", "low_c": "23", "low_f": "74", "condition": "Moderate rain", "date": "2023-10-03", "sunset": "07:14 PM", "sunrise": "07:26 AM" }, "urlstatus": 200, "forecast_information": { "forecast_date": "02:26 AM" }, "weather_update_recent": "02:27 UTC" },
  "testflight": false,
  "dcode": "DAL",
  "skymall": {},
  "dest_eta": "",
  "ocity": "Austin",
  "lon": "-106.458",
  "lond": 106,
  "percent_flt_complete": 51,
  "latd": 34,
  "latm": 24,
  "ttg_short": "00:00",
  "lons": 28,
  "dist_remain": "564",
  "ocode": "AUS",
  "updated": 1696041998.957196,
  "gspd": "553 mph",
  "pcent_flt_complete": 51,
  "dest_weather": { "dest_code": "KDAL", "forecast_information": { "forecast_date": "02:26 AM" }, "success": true, "fetch_time": 1696040896, "current_conditions": { "code": "113", "moonset": "07:29 AM", "temp_f": "86", "temp_c": "30", "humidity": "48", "sunset24": "19:15", "sunset": "07:15 PM", "sunrise": "07:20 AM", "sunrise24": "07:20", "condition": "Clear" }, "forecast_conditions_2": { "code": "113", "high_f": "96", "moonset": "08:38 AM", "high_c": "35", "day_of_week": "Saturday", "low_c": "25", "low_f": "76", "condition": "Sunny", "date": "2023-09-30", "sunset": "07:14 PM", "sunrise": "07:21 AM" }, "forecast_conditions_1": { "code": "113", "high_f": "99", "moonset": "07:29 AM", "high_c": "37", "day_of_week": "Friday", "low_c": "24", "low_f": "76", "condition": "Sunny", "date": "2023-09-29", "sunset": "07:15 PM", "sunrise": "07:20 AM" }, "weather_update_cnt": 13, "forecast_conditions_3": { "code": "113", "high_f": "94", "moonset": "09:48 AM", "high_c": "35", "day_of_week": "Sunday", "low_c": "24", "low_f": "76", "condition": "Sunny", "date": "2023-10-01", "sunset": "07:12 PM", "sunrise": "07:22 AM" }, "forecast_conditions_4": { "code": "113", "high_f": "97", "moonset": "10:57 AM", "high_c": "36", "day_of_week": "Monday", "low_c": "25", "low_f": "76", "condition": "Sunny", "date": "2023-10-02", "sunset": "07:11 PM", "sunrise": "07:22 AM" }, "forecast_conditions_5": { "code": "113", "high_f": "91", "moonset": "12:04 PM", "high_c": "33", "day_of_week": "Tuesday", "low_c": "25", "low_f": "77", "condition": "Sunny", "date": "2023-10-03", "sunset": "07:10 PM", "sunrise": "07:23 AM" }, "urlstatus": 200, "weather_update_recent": "02:28 UTC" },
  "octime24": "21:46",
  "gspdVal": "553",
  "ocitycode": "Austin",
  "ocountry": "United States",
  "ostate": "TX",
  "oiata": "AUS",
  "airline_iata": "wn",
  "fltnum": "9999",
  "fltleg": "10",
  "satcomm_status": { "commlink": "active", "linkparams": "active" },
  "actime": "09:46 PM",
  "routelock": "on",
  "airline_id": "swa",
  "actime24": "21:46",
  "icao24b": "unknown",
  "otzone": "CDT",
  "oicao": "KAUS",
  "connections": { "fltnum": "9999", "err": "Flight information missing data.", "code": 200, "success": false, "fetch_time": 1696041957, "dest": "DAL", "orig": "AUS" },
  "pricing": "",
  "tailnum": "N8716B",
  "wow": "offgnd",
  "within_us": true,
  "lochist": null,
  "current": 1696041998,
  "headingStr": "East",
  "dcountry": "United States",
  "smu_sn": "200630261",
  "headingImage": "url(\"/project_ext/flight/compass_E.png\")",
  "flight_start": { "tailnum": "N8716B", "start_ts": "2023093000", "orig": "PHX" },
  "sat_commlink_portal": { "status": "conn_ok", "time": "Sat Sep 30 02:28:27 2023" },
  "dicao": "KDAL",
  "dcitycode": "Dallas",
  "altVal": "37005",
  "ac_route_states": { "usa50route": true, "below10k": false, "hawaii": false, "thru_airport_nonusa": false, "below10k_dir": null, "lower48_gsm": true, "lower48": true, "descending_10k_5mins": false },
  "dcity": "Dallas",
  "airline_name": "Southwest Airlines",
  "lat": "34.405",
  "lonm": 27,
  "diata": "DAL",
  "offgnd_utc": 1696038901,
  "altm": "11 279 m",
  "lats": 18,
  "offgnd": 1696038901,
  "sat_cvg_state": {},
  "headingVal": "89.247"
}

    Sample:

    {
      "sat_commlink_portal": {
        "status": "conn_ok",
        "time": "Sat Sep 30 02:28:27 2023"
      },
      "pcent_flt_complete": 51,
      "altVal": "36998",
      "lon": "-107.664",
      "satcomm_status": {
        "commlink": "active",
        "linkparams": "active"
      },
      "dtzone": "CDT",
      "within_us": true,
      "etad": "07:00 PM",
      "lat": "34.387",
      "gspdVal": "556",
      "ttgc": "",
      "dist_remain": "632",
      "actime24": "21:39"
    }

*/

#[derive(Debug, Deserialize)]
pub struct FlightStats {
    #[serde(alias="pcent_flt_complete")]
    percent_flight_complete: i32,

    #[serde(alias="within_us")]
    within_united_states: bool,

    #[serde(alias="etad")]
    est_time_destination: Option<String>,


    #[serde(alias="altVal")]
    altitude_value: String,

    #[serde(alias="dcity")]
    destination_city_code: Option<String>,

    airline_name: Option<String>,

    #[serde(alias="lon")]
    longitude: String,

    #[serde(alias="lat")]
    latitude: String,

    dist_remain: String,
}
