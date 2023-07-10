# WL Real-Time Data API Rust Crate

## Disclaimer
**NOTE:** This project is not affiliated, endorsed, or supported by Wiener Linien or Stadt Wien. It is an independent effort to provide a Rust crate for interacting with the Wiener Linien Real-Time OGD API. Use this crate at your own discretion.

## Overview
This Rust crate provides a convenient interface for interacting with the Wiener Linien Real-Time Open Government Data (OGD) API. The API allows you to access real-time data about public transportation in Vienna, Austria. You can retrieve information about current vehicle positions, arrival times, departures, and more.

## Required Attribution
OGD data is licenced as CC BY 4.0 (https://creativecommons.org/licenses/by/4.0/deed.de), please see this page for required attributions in your application: https://www.data.gv.at/katalog/dataset/522d3045-0b37-48d0-b868-57c99726b1c4
  
From personal experience "Datenquelle: Stadt Wien - https://data.wien.gv.at" has been accepted in the past as sufficent attribution but your milage may vary.

## Example
For explanation of arguments of any Request please see the official WL documentation: https://www.wienerlinien.at/ogd_realtime/doku/ogd/wienerlinien-echtzeitdaten-dokumentation.pdf as this crates just directly maps to those interfaces.

### Monitor request
```rust
async fn send_monitor_req() {
    let mut request = MonitorRequest::new();
    request.stop_id.push(399);
    request.stop_id.push(5410);
    request
        .activate_traffic_info
        .push(ExtTrafficInfoEnum::TrafficInfo(
            TrafficInfoEnum::FahrtreppenInfo,
        ));
    request
        .activate_traffic_info
        .push(ExtTrafficInfoEnum::TrafficInfo(
            TrafficInfoEnum::AufzugsInfo,
        ));
    let resp = request.run().await;
    println!("{:?}", resp.ok());
}
```

### Traffic info
```rust
async fn send_traffic_info_req() {
    let mut request = TrafficInfoListRequest::new();
    let resp = request.run().await;
    println!("{:?}", resp.ok());
}
```

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
