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

    #[serde(alias="altVal")]
    altitude_value: String,

    #[serde(alias="lon")]
    longitude: String,

    #[serde(alias="lat")]
    latitude: String,

    dist_remain: String,
}
