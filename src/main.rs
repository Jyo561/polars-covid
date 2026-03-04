mod downloader;
mod processor;
mod utils;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_path = "data/raw/owid-covid-data.csv";
    let processed_path = "data/processed/total_cases_2022.csv";

    // Step 1: Download CSV
    downloader::download_covid_csv(raw_path)?;

    // Step 2: Load CSV
    let df = processor::load_csv(raw_path)?;

    // Step 3: Filter 2022
    let df_2022 = processor::filter_2022(&df)?;

    // Step 4: Aggregate total cases by country
    let total_cases = processor::total_cases_by_country(&df_2022)?;

    // Step 5: Save processed CSV
    total_cases.write_csv(processed_path)?;

    println!("Analysis complete. Results saved in {}", processed_path);
    Ok(())
}
