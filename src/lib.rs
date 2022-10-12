use chrono::prelude::*;

#[derive(Debug)]
pub struct Forecast {
    pub id: u64,
    pub locality: String,
    pub height: u16,
    pub date: DateTime<Local>,
    pub days: Vec<Day>,
}

#[derive(Debug)]
pub struct Day {
    pub date: NaiveDate,
    pub temperature_max: i64,
    pub temperature_min: i64,
    pub description: String,
    pub time_ranges: Vec<TimeRange>,
}

#[derive(Debug)]
pub struct TimeRange {
    pub time_range: String,
    pub brief_description: String,
    pub rain_probability: u8,
    pub rain_intensity: u8,
    pub freezing_level: u16,
}

/// Fetch weather data from meteotrentino site
pub fn fetch_weather_data(locality: &str) -> Result<Forecast, reqwest::Error> {
    let base_url = String::from("https://www.meteotrentino.it/protcivtn-meteo/api/front/previsioneOpenDataLocalita?localita=");
    let body = reqwest::blocking::get(base_url + locality)?.text()?;

    let data: serde_json::Value = serde_json::from_str(&body).unwrap();

    let mut days: Vec<Day> = Vec::new();

    // Iterate through all days
    for day_raw in data["previsione"][0]["giorni"].as_array().unwrap() {
        let mut time_ranges: Vec<TimeRange> = Vec::new();

        // Iterate through all time ranges
        for time_range_raw in day_raw["fasce"].as_array().unwrap() {
            // Push time range to vector
            time_ranges.push(TimeRange {
                time_range: time_range_raw["fasciaOre"].to_string(),
                rain_probability: time_range_raw["idPrecProb"]
                    .as_str()
                    .unwrap()
                    .parse::<u8>()
                    .unwrap(),
                rain_intensity: time_range_raw["idPrecInten"]
                    .as_str()
                    .unwrap()
                    .parse::<u8>()
                    .unwrap(),
                freezing_level: time_range_raw["zeroTermico"].as_u64().unwrap() as u16,
                brief_description: time_range_raw["descIcona"].to_string(),
            });
        }

        // Push day to vector
        days.push(Day {
            date: NaiveDate::parse_from_str(day_raw["giorno"].as_str().unwrap(), "%Y-%m-%d")
                .unwrap(),
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
            time_ranges,
        });
    }

    Ok(Forecast {
        id: data["idPrevisione"].as_u64().unwrap(),
        date: Local
            .datetime_from_str(
                data["dataPubblicazione"].as_str().unwrap(),
                "%Y-%m-%dT%H:%M%z",
            )
            .unwrap(),
        days,
        locality: data["previsione"][0]["localita"].to_string(),
        height: data["previsione"][0]["quota"].as_u64().unwrap() as u16,
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
