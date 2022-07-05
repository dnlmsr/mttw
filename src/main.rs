use reqwest;
use serde_json;

fn get_weather_data() -> Result<String, reqwest::Error> {
    let body = reqwest::blocking::get("https://www.meteotrentino.it/protcivtn-meteo/api/front/previsioneOpenDataLocalita?localita=TRENTO")?
        .text()?;
    Ok(body)
}

fn deserialize_json(data: String) -> serde_json::Result<serde_json::Value> {
   serde_json::from_str(&data)
}

fn main() {
    println!("{:?}",deserialize_json(get_weather_data().unwrap()).unwrap());
}
