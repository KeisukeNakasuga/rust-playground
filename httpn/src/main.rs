// see https://open-meteo.com/

trait ApiRequest {
    fn url(&self) -> String;
}

trait ApiGetMethod {
    fn get<Req: ApiRequest>(&self, req: Req) -> Result<String, reqwest::Error>;
}

struct OpenMeteoWeatherForecastApiRequest {
    latitude: f32,
    longitude: f32
}

impl ApiRequest for OpenMeteoWeatherForecastApiRequest {
    fn url(&self) -> String {
        format!("{}?latitude={}&longitude={}", OpenMeteoWeatherForecastApi::BASE_URL, self.latitude, self.longitude)
    }
}

struct OpenMeteoWeatherForecastApi;

impl OpenMeteoWeatherForecastApi {
    const BASE_URL: &'static str = "https://api.open-meteo.com/v1/forecast";
}

impl ApiGetMethod for OpenMeteoWeatherForecastApi {
    fn get<Req: ApiRequest>(&self, req: Req) -> Result<String, reqwest::Error> {
        let url = req.url();
        let res = reqwest::blocking::get(&url)?.text()?;
        Ok(res)
    }
}

fn main() {
    // tokyo
    let req = OpenMeteoWeatherForecastApiRequest { latitude: 35.6895, longitude: 139.6917 };
    let api = OpenMeteoWeatherForecastApi;
    let res = api.get(req);

    match res {
        Ok(res) => println!("Response: {}", res),
        Err(e) => println!("Error: {}", e),
    }
}
