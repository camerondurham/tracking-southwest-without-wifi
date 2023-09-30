# southwest flight stats without paying for internet

Based on the Hacker News post: https://jamesbvaughan.com/southwest-wifi/

Just dumps flight data to a local directory. I'll want to figure out how to make
something useful of the data later.

This output is incomplete, maybe when there's no connection to some internal service to provide weather info?

```json
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
```

This  example shows much more information and stats about your flight:

```json
{
  "dist_remaink": 952,
  "headingAbbrStr": "E",
  "dtzone": "CDT",
  "lonew": "West",
  "latns": "North",
  "public_ssid": "unknown",
  "dstate": "TX",
  "gspdm": "888 kph",
  "airline_icao": "swa",
  "alt": "37 006 ft",
  "comproute": false,
  "octime": "09:43 PM",
  "orig_weather": {
    "orig_code": "KAUS",
    "success": true,
    "fetch_time": 1696040873,
    "current_conditions": {
      "code": "116",
      "moonset": "07:32 AM",
      "temp_f": "86",
      "temp_c": "30",
      "humidity": "57",
      "sunset24": "19:19",
      "sunset": "07:19 PM",
      "sunrise": "07:23 AM",
      "sunrise24": "07:23",
      "condition": "Partly cloudy"
    },
    "forecast_conditions_2": {
      "code": "176",
      "high_f": "97",
      "moonset": "08:40 AM",
      "high_c": "36",
      "day_of_week": "Saturday",
      "low_c": "25",
      "low_f": "76",
      "condition": "Patchy rain possible",
      "date": "2023-09-30",
      "sunset": "07:18 PM",
      "sunrise": "07:24 AM"
    },
    "forecast_conditions_1": {
      "code": "113",
      "high_f": "99",
      "moonset": "07:32 AM",
      "high_c": "37",
      "day_of_week": "Friday",
      "low_c": "24",
      "low_f": "76",
      "condition": "Sunny",
      "date": "2023-09-29",
      "sunset": "07:19 PM",
      "sunrise": "07:23 AM"
    },
    "weather_update_cnt": 12,
    "forecast_conditions_3": {
      "code": "176",
      "high_f": "95",
      "moonset": "09:48 AM",
      "high_c": "35",
      "day_of_week": "Sunday",
      "low_c": "23",
      "low_f": "73",
      "condition": "Patchy rain possible",
      "date": "2023-10-01",
      "sunset": "07:17 PM",
      "sunrise": "07:25 AM"
    },
    "forecast_conditions_4": {
      "code": "113",
      "high_f": "94",
      "moonset": "10:55 AM",
      "high_c": "34",
      "day_of_week": "Monday",
      "low_c": "23",
      "low_f": "73",
      "condition": "Sunny",
      "date": "2023-10-02",
      "sunset": "07:15 PM",
      "sunrise": "07:25 AM"
    },
    "forecast_conditions_5": {
      "code": "302",
      "high_f": "89",
      "moonset": "12:01 PM",
      "high_c": "32",
      "day_of_week": "Tuesday",
      "low_c": "23",
      "low_f": "74",
      "condition": "Moderate rain",
      "date": "2023-10-03",
      "sunset": "07:14 PM",
      "sunrise": "07:26 AM"
    },
    "urlstatus": 200,
    "forecast_information": {
      "forecast_date": "02:26 AM"
    },
    "weather_update_recent": "02:27 UTC"
  },
  "testflight": false,
  "dcode": "DAL",
  "skymall": {},
  "dest_eta": "",
  "ocity": "Austin",
  "lon": "-107.019",
  "lond": 107,
  "percent_flt_complete": 51,
  "latd": 34,
  "latm": 23,
  "ttg_short": "00:00",
  "lons": 8,
  "dist_remain": "595",
  "ocode": "AUS",
  "updated": 1696041788.772574,
  "gspd": "552 mph",
  "pcent_flt_complete": 51,
  "dest_weather": {
    "dest_code": "KDAL",
    "forecast_information": {
      "forecast_date": "02:26 AM"
    },
    "success": true,
    "fetch_time": 1696040896,
    "current_conditions": {
      "code": "113",
      "moonset": "07:29 AM",
      "temp_f": "86",
      "temp_c": "30",
      "humidity": "48",
      "sunset24": "19:15",
      "sunset": "07:15 PM",
      "sunrise": "07:20 AM",
      "sunrise24": "07:20",
      "condition": "Clear"
    },
    "forecast_conditions_2": {
      "code": "113",
      "high_f": "96",
      "moonset": "08:38 AM",
      "high_c": "35",
      "day_of_week": "Saturday",
      "low_c": "25",
      "low_f": "76",
      "condition": "Sunny",
      "date": "2023-09-30",
      "sunset": "07:14 PM",
      "sunrise": "07:21 AM"
    },
    "forecast_conditions_1": {
      "code": "113",
      "high_f": "99",
      "moonset": "07:29 AM",
      "high_c": "37",
      "day_of_week": "Friday",
      "low_c": "24",
      "low_f": "76",
      "condition": "Sunny",
      "date": "2023-09-29",
      "sunset": "07:15 PM",
      "sunrise": "07:20 AM"
    },
    "weather_update_cnt": 13,
    "forecast_conditions_3": {
      "code": "113",
      "high_f": "94",
      "moonset": "09:48 AM",
      "high_c": "35",
      "day_of_week": "Sunday",
      "low_c": "24",
      "low_f": "76",
      "condition": "Sunny",
      "date": "2023-10-01",
      "sunset": "07:12 PM",
      "sunrise": "07:22 AM"
    },
    "forecast_conditions_4": {
      "code": "113",
      "high_f": "97",
      "moonset": "10:57 AM",
      "high_c": "36",
      "day_of_week": "Monday",
      "low_c": "25",
      "low_f": "76",
      "condition": "Sunny",
      "date": "2023-10-02",
      "sunset": "07:11 PM",
      "sunrise": "07:22 AM"
    },
    "forecast_conditions_5": {
      "code": "113",
      "high_f": "91",
      "moonset": "12:04 PM",
      "high_c": "33",
      "day_of_week": "Tuesday",
      "low_c": "25",
      "low_f": "77",
      "condition": "Sunny",
      "date": "2023-10-03",
      "sunset": "07:10 PM",
      "sunrise": "07:23 AM"
    },
    "urlstatus": 200,
    "weather_update_recent": "02:28 UTC"
  },
  "octime24": "21:43",
  "gspdVal": "552",
  "ocitycode": "Austin",
  "ocountry": "United States",
  "ostate": "TX",
  "oiata": "AUS",
  "airline_iata": "wn",
  "fltnum": "9999",
  "fltleg": "10",
  "satcomm_status": {
    "commlink": "active",
    "linkparams": "active"
  },
  "actime": "09:43 PM",
  "routelock": "on",
  "airline_id": "swa",
  "actime24": "21:43",
  "icao24b": "unknown",
  "otzone": "CDT",
  "oicao": "KAUS",
  "connections": {
    "fltnum": "9999",
    "err": "Flight information missing data.",
    "code": 200,
    "success": false,
    "fetch_time": 1696041770,
    "dest": "DAL",
    "orig": "AUS"
  },
  "pricing": "",
  "tailnum": "N8716B",
  "wow": "offgnd",
  "within_us": true,
  "lochist": null,
  "current": 1696041788,
  "headingStr": "East",
  "dcountry": "United States",
  "smu_sn": "200630261",
  "headingImage": "url(\"/project_ext/flight/compass_E.png\")",
  "flight_start": {
    "tailnum": "N8716B",
    "start_ts": "2023093000",
    "orig": "PHX"
  },
  "sat_commlink_portal": {
    "status": "conn_ok",
    "time": "Sat Sep 30 02:28:27 2023"
  },
  "dicao": "KDAL",
  "dcitycode": "Dallas",
  "altVal": "37006",
  "ac_route_states": {
    "usa50route": true,
    "below10k": false,
    "hawaii": false,
    "thru_airport_nonusa": false,
    "below10k_dir": null,
    "lower48_gsm": true,
    "lower48": true,
    "descending_10k_5mins": false
  },
  "dcity": "Dallas",
  "airline_name": "Southwest Airlines",
  "lat": "34.393",
  "lonm": 1,
  "diata": "DAL",
  "offgnd_utc": 1696038901,
  "altm": "11 279 m",
  "lats": 34,
  "offgnd": 1696038901,
  "sat_cvg_state": {},
  "headingVal": "91.500"
}
```
