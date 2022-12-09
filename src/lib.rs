use chrono::prelude::*;
use dirs::config_dir;
use serde_derive::Deserialize;
use toml;

#[derive(Deserialize)]
struct Config {
    default: ConfigDefault,
}

#[derive(Deserialize)]
struct ConfigDefault {
    locality: String,
}

/// The main forecast structure which contains all informations parsed from the official Meteotrentino data
#[derive(Debug)]
pub struct Forecast {
    /// Forecast ID
    pub id: u64,

    /// The specified locality
    pub locality: String,

    /// The height of the locality expressed in meters above sea level
    pub height: u16,

    /// The forecast date and local time
    pub date: DateTime<Local>,

    /// Vector of available upcoming days
    pub days: Vec<Day>,
}

#[derive(Debug)]
/// This structure contains information regarding the forecast during a specific day
pub struct Day {
    /// The date of the day
    pub date: NaiveDate,

    /// The maximum temperature expressed in °C
    pub temperature_max: i8,

    /// The minimum temperature expressed in °C
    pub temperature_min: i8,

    /// A long description of the weather conditions
    pub description: String,

    /// Vector of all available time ranges
    pub time_ranges: Vec<TimeRange>,
}

/// This structure contains information regarding the forecast of a given time range.
#[derive(Debug)]
pub struct TimeRange {
    /// The time range expressed in hh-hh
    pub time_range: String,

    /// A brief description associated to the icon
    pub brief_description: String,

    /// Rain probability expressed with a number between 1 and 4
    pub rain_probability: i8,

    /// Rain intensity expressed with a number between 1 and 4
    pub rain_intensity: i8,

    /// The freezing level expressed in meters above sea level
    pub freezing_level: u16,

    /// The snow altitude expressed in meters above sea level
    pub snow_altitude: Option<u64>,
}

/// Build Forecast struct from raw data
fn build_weather_data(body: &str) -> serde_json::Result<Forecast> {
    let raw_data: serde_json::Value = serde_json::from_str(&body)?;

    let mut days: Vec<Day> = Vec::new();

    // Iterate through all days
    for day_raw in raw_data["previsione"][0]["giorni"].as_array().unwrap() {
        let mut time_ranges: Vec<TimeRange> = Vec::new();

        // Iterate through all time ranges
        for time_range_raw in day_raw["fasce"].as_array().unwrap() {
            // Push time range to vector
            time_ranges.push(TimeRange {
                time_range: time_range_raw["fasciaOre"].to_string(),
                rain_probability: time_range_raw["idPrecProb"]
                    .as_str()
                    .unwrap()
                    .parse::<i8>()
                    .unwrap(),
                rain_intensity: time_range_raw["idPrecInten"]
                    .as_str()
                    .unwrap()
                    .parse::<i8>()
                    .unwrap(),
                freezing_level: time_range_raw["zeroTermico"].as_u64().unwrap() as u16,
                snow_altitude: time_range_raw["limiteNevicate"].as_u64(),
                brief_description: time_range_raw["descIcona"].to_string(),
            });
        }

        // Push day to vector
        days.push(Day {
            date: NaiveDate::parse_from_str(day_raw["giorno"].as_str().unwrap(), "%Y-%m-%d")
                .unwrap(),
            temperature_max: day_raw["tMaxGiorno"].as_i64().unwrap() as i8,
            temperature_min: day_raw["tMinGiorno"].as_i64().unwrap() as i8,
            description: String::from(day_raw["testoGiorno"].as_str().unwrap()),
            time_ranges,
        });
    }

    Ok(Forecast {
        id: raw_data["idPrevisione"].as_u64().unwrap(),
        date: Local
            .datetime_from_str(
                raw_data["dataPubblicazione"].as_str().unwrap(),
                "%Y-%m-%dT%H:%M%z",
            )
            .unwrap(),
        days,
        locality: raw_data["previsione"][0]["localita"].to_string(),
        height: raw_data["previsione"][0]["quota"].as_u64().unwrap() as u16,
    })
}

fn read_config() -> Option<Config> {
    let mut filename = config_dir().unwrap();
    filename.push("mttw");
    filename.push("config.toml");

    let contents = match std::fs::read_to_string(&filename) {
        Ok(data_raw) => data_raw,
        Err(_) => return None,
    };

    match toml::from_str(&contents) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

/// Fetch weather data from meteotrentino site
pub fn fetch_weather_data(locality: &Option<String>) -> Result<Forecast, reqwest::Error> {
    let config: Config = read_config().unwrap();
    let locality = match locality {
        Some(c) => c,
        None => config.default.locality.as_str(),
    };
    let base_url = String::from("https://www.meteotrentino.it/protcivtn-meteo/api/front/previsioneOpenDataLocalita?localita=");
    let body = reqwest::blocking::get(base_url + locality)?.text()?;
    Ok(build_weather_data(&body).unwrap())
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
