use std::{fs::File, io::Read};

use wl_realtime_ogd::models::{
    BuildRequestUrl, TrafficInfoEnum, TrafficInfoListRequest, TrafficInfoListResponse,
};

#[test]
fn test_build_request_url_empty() {
    let request = TrafficInfoListRequest::new();
    assert_eq!("/trafficInfoList?", request.build_request_url());
}

#[test]
fn test_build_request_url_with_related_line() {
    let mut request = TrafficInfoListRequest::new();
    request.related_line.push(String::from("A1"));
    assert_eq!(
        "/trafficInfoList?relatedLine=A1",
        request.build_request_url()
    );
}

#[test]
fn test_build_request_url_with_multiple_related_lines() {
    let mut request = TrafficInfoListRequest::new();
    request.related_line.push(String::from("A1"));
    request.related_line.push(String::from("A2"));
    assert_eq!(
        "/trafficInfoList?relatedLine=A1&relatedLine=A2",
        request.build_request_url()
    );
}

#[test]
fn test_build_request_url_with_related_stop() {
    let mut request = TrafficInfoListRequest::new();
    request.related_stop.push(1);
    assert_eq!(
        "/trafficInfoList?relatedStop=1",
        request.build_request_url()
    );
}

#[test]
fn test_build_request_url_with_multiple_related_stops() {
    let mut request = TrafficInfoListRequest::new();
    request.related_stop.push(1);
    request.related_stop.push(2);
    assert_eq!(
        "/trafficInfoList?relatedStop=1&relatedStop=2",
        request.build_request_url()
    );
}

#[test]
fn test_build_request_url_with_name() {
    let mut request = TrafficInfoListRequest::new();
    request.name.push(TrafficInfoEnum::StoerungLang);
    assert_eq!(
        "/trafficInfoList?name=stoerunglang",
        request.build_request_url()
    );
}

#[test]
fn test_build_request_url_with_multiple_names() {
    let mut request = TrafficInfoListRequest::new();
    request.name.push(TrafficInfoEnum::StoerungLang);
    request.name.push(TrafficInfoEnum::AufzugsInfo);
    assert_eq!(
        "/trafficInfoList?name=stoerunglang&name=aufzugsinfo",
        request.build_request_url()
    );
}

#[test]
fn test_build_request_url_with_related_line_stop_and_name() {
    let mut request = TrafficInfoListRequest::new();
    request.related_line.push(String::from("A1"));
    request.related_stop.push(1);
    request.name.push(TrafficInfoEnum::StoerungLang);
    assert_eq!(
        "/trafficInfoList?relatedLine=A1&relatedStop=1&name=stoerunglang",
        request.build_request_url()
    );
}

#[test]
fn test_build_request_url_with_multiple_related_lines_stops_and_names() {
    let mut request = TrafficInfoListRequest::new();
    request.related_line.push(String::from("A1"));
    request.related_line.push(String::from("A2"));
    request.related_stop.push(1);
    request.related_stop.push(2);
    request.name.push(TrafficInfoEnum::StoerungLang);
    request.name.push(TrafficInfoEnum::AufzugsInfo);
    assert_eq!(
        "/trafficInfoList?relatedLine=A1&relatedLine=A2&relatedStop=1&relatedStop=2&name=stoerunglang&name=aufzugsinfo",
        request.build_request_url()
    );
}

fn get_mock_data() -> String {
    let mut file = File::open("./tests/assets/traffic-info-list.json").unwrap();
    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

#[test]
fn test_parse_traffic_info_list_response() {
    let buffer = get_mock_data();
    let response: TrafficInfoListResponse = serde_json::from_str(&buffer).unwrap();
    assert_eq!(response.data.traffic_infos.unwrap().len(), 17);
    assert_eq!(response.data.traffic_info_categories.unwrap().len(), 2);
    assert_eq!(response.data.traffic_info_category_groups.unwrap().len(), 1);
}
