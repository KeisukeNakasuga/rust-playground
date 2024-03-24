// see https://open-meteo.com/

use std::error::Error;
use reqwest;
use serde::{Deserialize};
use serde_json;

trait ApiRequest {
    fn url(&self) -> String;
}

trait HttpGet<Res> {
    fn get<Req: ApiRequest>(&self, req: Req) -> Result<Res, Box<dyn Error>>;
}

struct OpenMeteoWeatherForecastApiRequest {
    latitude: f32,
    longitude: f32
}

impl ApiRequest for OpenMeteoWeatherForecastApiRequest {
    fn url(&self) -> String {
        format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}", self.latitude, self.longitude)
    }
}

#[derive(Deserialize)]
struct OpenMeteoWeatherForecastApiResponse {
    latitude: f32,
    longitude: f32,
    generationtime_ms: f32,
    utc_offset_seconds: i8,
    timezone: String,
    timezone_abbreviation: String,
    elevation:f32,
}

struct OpenMeteoWeatherForecastApi;

impl HttpGet<OpenMeteoWeatherForecastApiResponse> for OpenMeteoWeatherForecastApi {
    fn get<Req: ApiRequest>(&self, req: Req) -> Result<OpenMeteoWeatherForecastApiResponse, Box<dyn Error>> {
        let url = req.url();
        let body = reqwest::blocking::get(&url)?.text()?;
        let obj = serde_json::from_str(&body)?;
        Ok(obj)
    }
}

fn main() {
    // tokyo
    let req = OpenMeteoWeatherForecastApiRequest { latitude: 35.6895, longitude: 139.6917 };
    let api = OpenMeteoWeatherForecastApi;
    let res = api.get(req);

    match res {
        Ok(res) => println!(
            "Response: latitude => {}, longitude => {}, generationtime_ms => {}, utc_offset_seconds => {}, timezone => {}, timezone_abbreviation => {}, elevation => {}",
            res.latitude, res.longitude, res.generationtime_ms, res.utc_offset_seconds, res.timezone, res.timezone_abbreviation, res.elevation),
        Err(e) => println!("Error: {}", e),
    }
}
