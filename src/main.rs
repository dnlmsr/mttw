use chrono::{Duration, Local};
use clap::{Parser, Subcommand};

/// Meteotrentino wrapper
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// Name of the locality
    #[clap(global = true, short, long, value_parser)]
    locality: Option<String>,

    /// CLI subcommands
    #[clap(subcommand)]
    command: Option<Commands>,
}

/// CLI subcommands
#[derive(Subcommand, Debug)]
enum Commands {
    /// Show today weather
    Today,

    /// Show tomorrow weather
    Tomorrow,
}

fn main() {
    let args = Args::parse();

    let forecast = mttw::fetch_weather_data(&args.locality).unwrap();

    println!("Weather forecast for: {}.", forecast.locality);
    {
        let time_now = Local::now().time();
        let time_difference = time_now - forecast.date.time();
        println!(
            "Last forecast update was {} hours and {} minutes ago.",
            time_difference.num_hours(),
            time_difference.num_minutes() % 60
        );
    }

    let now = Local::now();

    let day = match &args.command {
        None | Some(Commands::Today) => forecast.get_day(&now.date_naive()).unwrap(),
        Some(Commands::Tomorrow) => forecast
            .get_day(&(now.date_naive() + Duration::days(1)))
            .unwrap(),
    };
    println!("\nDay forecast");
    println!("Max. temperature: {}°C", day.temperature_max);
    println!("Min temperature: {}°C", day.temperature_min);
    println!("Description: {}", day.description);

    println!("\nTime range forecast");
    println!("Brief description {}", day.time_ranges[0].brief_description);
    println!("Rain probability: {}", day.time_ranges[0].rain_probability);
    println!("Rain intensity: {}", day.time_ranges[0].rain_intensity);
    println!("Freezing level: {}m", day.time_ranges[0].freezing_altitude);
    if let Some(snow_altitude) = day.time_ranges[0].snow_altitude {
        println!("Snow altitude: {snow_altitude}m");
    }
}
