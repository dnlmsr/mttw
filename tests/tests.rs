#[test]
fn test_correct_forecast_names() {
    let localities = vec![
        "TRENTO",
        "Trento",
        "Trento  ",
        "trento",
        "rifugio pedrotti  alla tosa",
    ];
    for locality in localities {
        let forecast = mttw::fetch_weather_data(locality).unwrap();
        assert!(forecast.days[0].temperature_max > forecast.days[0].temperature_min);
    }
}

#[test]
#[should_panic]
fn test_incorrect_forecast_names() {
    let locality = "undefined";
    let forecast = mttw::fetch_weather_data(locality).unwrap();
    println!("{}", forecast.days[0].description)
}

#[test]
fn test_correct_temperatures() {
    let locality = "TRENTO";
    let forecast = mttw::fetch_weather_data(locality).unwrap();
    assert!(forecast.days[0].temperature_max > forecast.days[0].temperature_min);
    assert!(forecast.days[0].temperature_min > -100);
    assert!(forecast.days[0].temperature_max < 100);
}

#[test]
fn test_correct_brief() {
    let locality = "TRENTO";
    let forecast = mttw::fetch_weather_data(locality).unwrap();
    assert!(forecast.days[0].description.chars().count() > 10);
}
