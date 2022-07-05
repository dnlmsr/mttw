use reqwest;
use serde_json;

fn get_weather_data() -> Result<serde_json::Value, reqwest::Error> {
    let body = reqwest::blocking::get("https://www.meteotrentino.it/protcivtn-meteo/api/front/previsioneOpenDataLocalita?localita=TRENTO")?
        .text()?;
    let deserialized: serde_json::Value = serde_json::from_str(&body).unwrap();
    Ok(deserialized)
}

fn main() {
    println!("{}",get_weather_data().unwrap());
}
