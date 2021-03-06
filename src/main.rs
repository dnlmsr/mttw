use clap::Parser;
use reqwest;
use serde_json;
use std::fs::File;

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
    description: String,
}

/// Fetch weather data from meteotrentino site
fn fetch_weather_data(locality: &String) -> Result<String, reqwest::Error> {
    let base_url = String::from("https://www.meteotrentino.it/protcivtn-meteo/api/front/previsioneOpenDataLocalita?localita=");
    let body = reqwest::blocking::get(base_url + locality)?.text()?;
    Ok(body)
}

/// Download icon and store it in cache
fn download_icon(icon_url: &str) {
    // Get icon full name
    let icon_filename = {
        let i = icon_url.rfind('/').unwrap() + 1;
        &icon_url[i..]
    };

    // Create icons directory if it doesn't exist
    let icons_directory = format!(
        "{}{}",
        std::env::var("HOME").unwrap(),
        "/.cache/mttw/icons/"
    );
    std::fs::create_dir_all(&icons_directory).expect("Unable to create directory");
    let icon_path = format!("{}{}", icons_directory, icon_filename);

    // Save icon if it doesn't exist
    if !std::path::Path::new(&icon_path).exists() {
        let mut file = File::create(icon_path).expect("Failed opening file");
        reqwest::blocking::get(icon_url)
            .unwrap()
            .copy_to(&mut file)
            .expect("Failed downloading image");
    }
}

/// Deserialize JSON data
fn deserialize_json(data: String) -> serde_json::Result<serde_json::Value> {
    serde_json::from_str(&data)
}

fn main() {
    let args = Args::parse();

    let data = deserialize_json(fetch_weather_data(&args.locality).unwrap()).unwrap();
    let forecast = Forecast {
        id: data["idPrevisione"].as_u64().unwrap(),
        temperature_max: data["previsione"][0]["giorni"][0]["tMaxGiorno"]
            .as_i64()
            .unwrap(),
        temperature_min: data["previsione"][0]["giorni"][0]["tMinGiorno"]
            .as_i64()
            .unwrap(),
        description: String::from(
            data["previsione"][0]["giorni"][0]["testoGiorno"]
                .as_str()
                .unwrap(),
        ),
    };

    download_icon(
        data["previsione"][0]["giorni"][0]["icona"]
            .as_str()
            .unwrap(),
    );

    println!("Weather forecast for: {}.", &args.locality);
    println!("Temperatura massima: {}??C", forecast.temperature_max);
    println!("Temperatura minima: {}??C", forecast.temperature_min);
    println!("Evoluzione: {}", forecast.description);
}
