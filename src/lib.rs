use chrono::{DateTime, FixedOffset};

#[derive(Debug)]
pub struct Forecast {
    pub id: u64,
    pub temperature_max: i64,
    pub temperature_min: i64,
    pub description: String,
    pub date: DateTime<FixedOffset>,
}

/// Fetch weather data from meteotrentino site
pub fn fetch_weather_data(locality: &String) -> Result<Forecast, reqwest::Error> {
    let base_url = String::from("https://www.meteotrentino.it/protcivtn-meteo/api/front/previsioneOpenDataLocalita?localita=");
    let body = reqwest::blocking::get(base_url + locality)?.text()?;

    let data: serde_json::Value = serde_json::from_str(&body).unwrap();

    Ok(Forecast {
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
        date: DateTime::parse_from_str(
            data["dataPubblicazione"].as_str().unwrap(),
            "%Y-%m-%dT%H:%M%z",
        )
        .unwrap(),
    })
}

/*
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
        let mut file = std::fs::File::create(icon_path).expect("Failed opening file");
        reqwest::blocking::get(icon_url)
            .unwrap()
            .copy_to(&mut file)
            .expect("Failed downloading image");
    }
}
*/
