use chrono::Local;
use clap::{Parser, Subcommand};

/// Meteotrentino wrapper
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// Name of the locality
    #[clap(global = true, short, long, value_parser)]
    locality: Option<String>,

    #[clap(subcommand)]
    command: Option<Commands>,
}

/// CLI commands
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

    let day = match &args.command {
        None | Some(Commands::Today) => 0,
        Some(Commands::Tomorrow) => 1,
    };
    println!("\nDay forecast");
    println!("Max. temperature: {}°C", forecast.days[day].temperature_max);
    println!("Min temperature: {}°C", forecast.days[day].temperature_min);
    println!("Description: {}", forecast.days[day].description);

    println!("\nTime range forecast");
    println!(
        "Brief description {}",
        forecast.days[day].time_ranges[0].brief_description
    );
    println!(
        "Rain probability: {}",
        forecast.days[day].time_ranges[0].rain_probability
    );
    println!(
        "Rain intensity: {}",
        forecast.days[day].time_ranges[0].rain_intensity
    );
    println!(
        "Freezing level: {}m",
        forecast.days[day].time_ranges[0].freezing_altitude
    );
    match forecast.days[day].time_ranges[0].snow_altitude {
        Some(snow_altitude) => println!("Snow altitude: {}m", snow_altitude),
        None => (),
    };
}
