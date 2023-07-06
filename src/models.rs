use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer, Serialize};
use std::{error::Error, fmt::Display};

use crate::{
    helpers::join_vec,
    serde_deserializers::{date_format, optional_date_format},
};

const WL_ENDPOINT: &str = "https://www.wienerlinien.at/ogd_realtime";

pub trait BuildRequestUrl {
    fn build_request_url(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct MonitorRequest {
    pub stop_id: Vec<u32>,
    pub diva: Option<u32>,
    pub activate_traffic_info: Vec<ExtTrafficInfoEnum>,
    pub a_area: bool,
}

impl MonitorRequest {
    pub fn new() -> Self {
        MonitorRequest {
            activate_traffic_info: vec![],
            stop_id: vec![],
            diva: None,
            a_area: false,
        }
    }

    pub async fn run(&self) -> Result<MonitorResponse, Box<dyn Error>> {
        let url = WL_ENDPOINT.to_owned() + &self.build_request_url();
        let response = reqwest::get(url).await?.json::<MonitorResponse>().await?;
        Ok(response)
    }
}

impl Default for MonitorRequest {
    fn default() -> Self {
        MonitorRequest::new()
    }
}

impl BuildRequestUrl for MonitorRequest {
    fn build_request_url(&self) -> String {
        let mut url = String::from("/monitor?");
        let mut url_segments: Vec<String> = vec![];
        if !self.stop_id.is_empty() {
            let mut stops = join_vec("stopId=", &self.stop_id);
            url_segments.append(&mut stops);
        }
        if self.diva.is_some() {
            url_segments.push(format!("diva={}", self.diva.unwrap()));
        }
        if !self.activate_traffic_info.is_empty() {
            let mut traffic_info = join_vec("activateTrafficInfo=", &self.activate_traffic_info);
            url_segments.append(&mut traffic_info);
        }
        if self.a_area {
            url_segments.push(String::from("aArea=1"));
        }

        url.push_str(&url_segments.join("&"));
        url
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrafficInfoListRequest {
    pub related_line: Vec<String>,
    pub related_stop: Vec<u32>,
    pub name: Vec<TrafficInfoEnum>,
}

impl TrafficInfoListRequest {
    pub fn new() -> Self {
        TrafficInfoListRequest {
            related_line: vec![],
            related_stop: vec![],
            name: vec![],
        }
    }

    pub async fn run(&self) -> Result<TrafficInfoListResponse, Box<dyn Error>> {
        let url = WL_ENDPOINT.to_owned() + &self.build_request_url();
        let response = reqwest::get(url)
            .await?
            .json::<TrafficInfoListResponse>()
            .await?;
        Ok(response)
    }
}

impl Default for TrafficInfoListRequest {
    fn default() -> Self {
        TrafficInfoListRequest::new()
    }
}

impl BuildRequestUrl for TrafficInfoListRequest {
    fn build_request_url(&self) -> String {
        let mut url = String::from("/trafficInfoList?");
        let mut url_segments: Vec<String> = vec![];
        if !self.related_line.is_empty() {
            let mut lines = join_vec("relatedLine=", &self.related_line);
            url_segments.append(&mut lines);
        }
        if !self.related_stop.is_empty() {
            let mut stops = join_vec("relatedStop=", &self.related_stop);
            url_segments.append(&mut stops);
        }
        if !self.name.is_empty() {
            let mut names = join_vec("name=", &self.name);
            url_segments.append(&mut names);
        }

        url.push_str(&url_segments.join("&"));
        url
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum TrafficInfoEnum {
    StoerungLang,
    StoerungKurz,
    AufzugsInfo,
    FahrtreppenInfo,
}

impl Display for TrafficInfoEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: &str = match self {
            TrafficInfoEnum::StoerungLang => "stoerunglang",
            TrafficInfoEnum::StoerungKurz => "stoerungkurz",
            TrafficInfoEnum::AufzugsInfo => "aufzugsinfo",
            TrafficInfoEnum::FahrtreppenInfo => "fahrtreppeninfo",
        };
        write!(f, "{}", str)
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum ExtTrafficInfoEnum {
    TrafficInfo(TrafficInfoEnum),
    Information,
}

impl Display for ExtTrafficInfoEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtTrafficInfoEnum::Information => write!(f, "information"),
            ExtTrafficInfoEnum::TrafficInfo(other) => other.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageCode {
    OK = 1,
    DbOffline = 311,
    StopDoesNotExist = 312,
    RequestLimitExceeded = 316,
    GetParamInvalid = 320,
    GetParamMissing = 321,
    NoDataFound = 322,
}

impl<'de> Deserialize<'de> for MessageCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code: u32 = Deserialize::deserialize(deserializer)?;

        match code {
            1 => Ok(MessageCode::OK),
            311 => Ok(MessageCode::DbOffline),
            312 => Ok(MessageCode::StopDoesNotExist),
            316 => Ok(MessageCode::RequestLimitExceeded),
            320 => Ok(MessageCode::GetParamInvalid),
            321 => Ok(MessageCode::GetParamMissing),
            322 => Ok(MessageCode::NoDataFound),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown message code: {}",
                code
            ))),
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Message {
    pub value: String,
    #[serde(rename = "messageCode")]
    pub message_code: MessageCode,
    #[serde(rename = "serverTime", with = "date_format")]
    pub server_time: DateTime<FixedOffset>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Response {
    pub message: Message,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub geometry_type: String,
    pub coordinates: Vec<f64>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Attributes {
    pub rbl: i32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Properties {
    pub name: String,
    pub title: String,
    pub municipality: String,
    #[serde(rename = "municipalityId")]
    pub municipality_id: i32,
    #[serde(rename = "type")]
    pub stop_type: String,
    #[serde(rename = "coordName")]
    pub coord_name: String,
    pub attributes: Attributes,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct LocationStop {
    #[serde(rename = "type")]
    pub location_type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DepartureTime {
    #[serde(rename = "timePlanned", with = "date_format")]
    pub time_planned: DateTime<FixedOffset>,
    #[serde(rename = "timeReal", default, with = "optional_date_format")]
    pub time_real: Option<DateTime<FixedOffset>>,
    pub countdown: i32,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Vehicle {
    pub name: String,
    pub towards: String,
    pub direction: String,
    #[serde(rename = "richtungsId")]
    pub richtungs_id: String,
    #[serde(rename = "barrierFree")]
    pub barrier_free: bool,
    #[serde(rename = "realtimeSupported")]
    pub realtime_supported: bool,
    pub trafficjam: bool,
    #[serde(rename = "type")]
    pub vehicle_type: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Departure {
    #[serde(rename = "departureTime")]
    pub departure_time: DepartureTime,
    pub vehicle: Option<Vehicle>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Departures {
    pub departure: Vec<Departure>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Line {
    pub name: String,
    pub towards: String,
    pub direction: String,
    #[serde(rename = "richtungsId")]
    pub richtungs_id: String,
    #[serde(rename = "barrierFree")]
    pub barrier_free: bool,
    #[serde(rename = "realtimeSupported")]
    pub realtime_supported: bool,
    pub trafficjam: bool,
    pub departures: Departures,
    #[serde(rename = "type")]
    pub line_type: String,
    #[serde(rename = "lineId")]
    pub line_id: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Monitor {
    #[serde(rename = "locationStop")]
    pub location_stop: LocationStop,
    pub lines: Vec<Line>,
    #[serde(rename = "refTrafficInfoNames")]
    pub ref_traffic_info_names: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Time {
    #[serde(default, with = "optional_date_format")]
    pub start: Option<DateTime<FixedOffset>>,
    #[serde(default, with = "optional_date_format")]
    pub end: Option<DateTime<FixedOffset>>,
    #[serde(default, with = "optional_date_format")]
    pub resume: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AttributesTrafficInfo {
    pub status: Option<String>,
    pub station: Option<String>,
    pub location: Option<String>,
    pub reason: Option<String>,
    pub towards: Option<String>,
    #[serde(rename = "relatedLines")]
    pub related_lines: Option<Vec<String>>,
    #[serde(rename = "relatedStops")]
    pub related_stops: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TrafficInfo {
    #[serde(rename = "refTrafficInfoCategoryId")]
    pub ref_traffic_info_category_id: i32,
    pub name: String,
    pub priority: Option<String>,
    pub owner: Option<String>,
    pub title: String,
    pub description: String,
    pub time: Option<Time>,
    pub attributes: Option<AttributesTrafficInfo>,
    #[serde(rename = "relatedLines")]
    pub related_lines: Option<Vec<String>>,
    #[serde(rename = "relatedStops")]
    pub related_stops: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TrafficInfoCategory {
    pub id: i32,
    #[serde(rename = "refTrafficInfoCategoryGroupId")]
    pub ref_traffic_info_category_group_id: i32,
    pub name: String,
    #[serde(rename = "trafficInfoNameList")]
    pub traffic_info_name_list: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TrafficInfoCategoryGroup {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct MonitorResponseData {
    pub monitors: Vec<Monitor>,
    #[serde(rename = "trafficInfos")]
    pub traffic_infos: Option<Vec<TrafficInfo>>,
    #[serde(rename = "trafficInfoCategories")]
    pub traffic_info_categories: Option<Vec<TrafficInfoCategory>>,
    #[serde(rename = "trafficInfoCategoryGroups")]
    pub traffic_info_category_groups: Option<Vec<TrafficInfoCategoryGroup>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct MonitorResponse {
    pub message: Message,
    pub data: MonitorResponseData,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TrafficInfoListResponseData {
    #[serde(rename = "trafficInfos")]
    pub traffic_infos: Option<Vec<TrafficInfo>>,
    #[serde(rename = "trafficInfoCategories")]
    pub traffic_info_categories: Option<Vec<TrafficInfoCategory>>,
    #[serde(rename = "trafficInfoCategoryGroups")]
    pub traffic_info_category_groups: Option<Vec<TrafficInfoCategoryGroup>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TrafficInfoListResponse {
    pub message: Message,
    pub data: TrafficInfoListResponseData,
}
