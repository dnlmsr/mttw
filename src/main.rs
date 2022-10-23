use chrono::Local;
use clap::{Parser, Subcommand};

/// Meteotrentino wrapper
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// Name of the locality, defaults to "TRENTO"
    #[clap(short, long, value_parser, default_value_t = String::from("TRENTO"))]
    locality: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show today weather
    Today {},
    Tomorrow {},
}

fn main() {
    let args = Args::parse();

    let forecast = mttw::fetch_weather_data(&args.locality).unwrap();

    println!("Weather forecast for: {}.", &args.locality);
    {
        let time_now = Local::now().time();
        let time_difference = time_now - forecast.date.time();
        println!(
            "Last forecast update was {} hours and {} minutes ago.",
            time_difference.num_hours(),
            time_difference.num_minutes() % 60
        );
    }
    println!("\nDay forecast");
    println!("Max. temperature: {}°C", forecast.days[0].temperature_max);
    println!("Min temperature: {}°C", forecast.days[0].temperature_min);
    println!("Description: {}", forecast.days[0].description);

    println!("\nTime range forecast");
    println!(
        "Brief description {}",
        forecast.days[0].time_ranges[0].brief_description
    );
    println!(
        "Rain probability: {}",
        forecast.days[0].time_ranges[0].rain_probability
    );
    println!(
        "Rain intensity: {}",
        forecast.days[0].time_ranges[0].rain_intensity
    );
    println!(
        "Freezing level: {}m",
        forecast.days[0].time_ranges[0].freezing_level
    );
}
