use std::fs::File;
use std::io::copy;
use std::error::Error;

pub fn download_covid_csv(path: &str) -> Result<(), Box<dyn Error>> {
    let url = "https://covid.ourworldindata.org/data/owid-covid-data.csv";
    let mut resp = reqwest::blocking::get(url)?.bytes()?;
    let mut out = File::create(path)?;
    copy(&mut resp.as_ref(), &mut out)?;
    println!("Downloaded CSV to {}", path);
    Ok(())
}
