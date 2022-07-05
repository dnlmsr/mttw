use curl::easy::Easy;
use std::io::{stdout, Write};

fn main() {

    let mut easy = Easy::new();
    easy.url("https://www.meteotrentino.it/protcivtn-meteo/api/front/previsioneOpenDataLocalita?localita=TRENTO").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    println!("{}",easy.response_code().unwrap());
}
