use reqwest;
use serde_json;
use clap::Parser;

/// Meteotrentino wrapper
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the locality
    #[clap(short, long, value_parser)]
    locality: String,
}

#[derive(Debug)]
struct Forecast {
    id: u64,
    temperature_max: i64,
    temperature_min: i64,
}

fn get_weather_data() -> Result<String, reqwest::Error> {
    let body = reqwest::blocking::get("https://www.meteotrentino.it/protcivtn-meteo/api/front/previsioneOpenDataLocalita?localita=TRENTO")?
        .text()?;
    Ok(body)
}

fn deserialize_json(data: String) -> serde_json::Result<serde_json::Value> {
   serde_json::from_str(&data)
}

fn main() {
    let data = deserialize_json(get_weather_data().unwrap()).unwrap();
    let forecast = Forecast{
        id: data["idPrevisione"].as_u64().unwrap(),
        temperature_max: data["previsione"][0]["giorni"][0]["tMaxGiorno"].as_i64().unwrap(),
        temperature_min: data["previsione"][0]["giorni"][0]["tMinGiorno"].as_i64().unwrap(),
    };

    let args = Args::parse();
    println!("{:?}",args);
}
