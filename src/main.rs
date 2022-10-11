use chrono::Utc;
use clap::Parser;

/// Meteotrentino wrapper
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the locality, defaults to "TRENTO"
    #[clap(short, long, value_parser, default_value_t = String::from("TRENTO"))]
    locality: String,
}

fn main() {
    let args = Args::parse();

    let forecast = mttw::fetch_weather_data(&args.locality).unwrap();

    println!("Weather forecast for: {}.", &args.locality);
    {
        let time_now = Utc::now().time();
        let time_difference = time_now - forecast.date.time();
        println!(
            "Last forecast update was {} hours and {} minutes ago.",
            time_difference.num_hours(),
            time_difference.num_minutes() % 60
        );
    }
    println!(
        "Temperatura massima: {}°C",
        forecast.days[0].temperature_max
    );
    println!("Temperatura minima: {}°C", forecast.days[0].temperature_min);
    println!("Description: {}", forecast.days[0].description);
}
