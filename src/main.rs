use reqwest;

fn foo() -> Result<(), reqwest::Error> {
    let body = reqwest::blocking::get("https://www.meteotrentino.it/protcivtn-meteo/api/front/previsioneOpenDataLocalita?localita=TRENTO")?
        .text()?;
    println!("body = {:?}", body);
    Ok(())
}

fn main() {
    foo();
}
