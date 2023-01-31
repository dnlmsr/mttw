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
        let forecast = mttw::fetch_weather_data(&Some(String::from(locality))).unwrap();
        assert!(forecast.days[0].temperature_max > forecast.days[0].temperature_min);
    }
}

#[test]
#[should_panic]
fn test_incorrect_forecast_names() {
    let locality = "undefined";
    let forecast = mttw::fetch_weather_data(&Some(String::from(locality))).unwrap();
    println!("{}", forecast.days[0].description)
}

#[test]
fn test_correct_temperatures() {
    let forecast = mttw::fetch_weather_data(&None).unwrap();
    assert!(forecast.days[0].temperature_max > forecast.days[0].temperature_min);
    assert!(forecast.days[0].temperature_min > -100);
    assert!(forecast.days[0].temperature_max < 100);
}

#[test]
fn test_correct_brief() {
    let forecast = mttw::fetch_weather_data(&None).unwrap();
    assert!(forecast.days[0].description.chars().count() > 10);
}

#[test]
fn test_other_days() {
    let forecast = mttw::fetch_weather_data(&None).unwrap();
    assert!(forecast.days.len() > 3);
    for day in forecast.days {
        assert!(day.temperature_max > day.temperature_min);
    }
}

#[test]
fn test_freezing_altitude() {
    let forecast = mttw::fetch_weather_data(&None).unwrap();
    assert!(forecast.days[0].time_ranges[0].freezing_altitude > 0);
}

#[test]
fn test_get_day() {
    let forecast = mttw::fetch_weather_data(&None).unwrap();
    let now = chrono::Local::now().date().naive_local();
    let day: &mttw::Day = forecast.get_day(&now).unwrap();
    assert_eq!(day.date.to_string(), now.to_string());
}
