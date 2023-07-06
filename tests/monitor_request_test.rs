use std::{fs::File, io::Read};

use wiener_linien_ogd::models::{
    BuildRequestUrl, ExtTrafficInfoEnum, MonitorRequest, MonitorResponse, TrafficInfoEnum,
};

#[test]
fn test_build_request_url_empty() {
    let request = MonitorRequest::new();
    assert_eq!("/monitor?", request.build_request_url());
}

#[test]
fn test_build_request_url_with_stop_id() {
    let mut request = MonitorRequest::new();
    request.stop_id.push(123);
    assert_eq!("/monitor?stopId=123", request.build_request_url());
}

#[test]
fn test_build_request_url_with_multiple_stop_ids() {
    let mut request = MonitorRequest::new();
    request.stop_id.push(123);
    request.stop_id.push(456);
    assert_eq!(
        "/monitor?stopId=123&stopId=456",
        request.build_request_url()
    );
}

#[test]
fn test_build_request_url_with_diva() {
    let mut request = MonitorRequest::new();
    request.diva = Some(999);
    assert_eq!("/monitor?diva=999", request.build_request_url());
}

#[test]
fn test_build_request_url_with_traffic_info() {
    let mut request = MonitorRequest::new();
    request
        .activate_traffic_info
        .push(ExtTrafficInfoEnum::TrafficInfo(
            TrafficInfoEnum::FahrtreppenInfo,
        ));
    assert_eq!(
        "/monitor?activateTrafficInfo=fahrtreppeninfo",
        request.build_request_url()
    );
}

#[test]
fn test_build_request_url_with_multiple_traffic_info() {
    let mut request = MonitorRequest::new();
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
    assert_eq!(
        "/monitor?activateTrafficInfo=fahrtreppeninfo&activateTrafficInfo=aufzugsinfo",
        request.build_request_url()
    );
}

#[test]
fn test_build_request_url_with_a_area() {
    let mut request = MonitorRequest::new();
    request.a_area = true;
    assert_eq!("/monitor?aArea=1", request.build_request_url());
}

#[test]
fn test_build_request_url_with_all_parameters() {
    let mut request = MonitorRequest::new();
    request.stop_id.push(123);
    request.stop_id.push(456);
    request.diva = Some(999);
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
    request.a_area = true;
    assert_eq!("/monitor?stopId=123&stopId=456&diva=999&activateTrafficInfo=fahrtreppeninfo&activateTrafficInfo=aufzugsinfo&aArea=1", request.build_request_url());
}

fn get_mock_data() -> String {
    let mut file = File::open("./tests/assets/monitor-response.json").unwrap();
    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

#[test]
fn test_parse_monitor_response() {
    let buffer = get_mock_data();
    let response: MonitorResponse = serde_json::from_str(&buffer).unwrap();
    assert_eq!(response.data.monitors.len(), 2);
}
