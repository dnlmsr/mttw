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
    println!("Temperatura massima: {}°C", forecast.temperature_max);
    println!("Temperatura minima: {}°C", forecast.temperature_min);
    println!("Evoluzione: {}", forecast.description);
}
