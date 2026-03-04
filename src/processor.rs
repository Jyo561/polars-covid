use polars::prelude::*;
use std::error::Error;

pub fn load_csv(path: &str) -> Result<DataFrame, Box<dyn Error>> {
    let df = CsvReader::from_path(path)?
        .infer_schema(Some(100))
        .has_header(true)
        .finish()?;
    println!("Loaded CSV from {}", path);
    Ok(df)
}

pub fn filter_2022(df: &DataFrame) -> Result<DataFrame, Box<dyn Error>> {
    let df_filtered = df
        .lazy()
        .filter(
            col("date")
                .gt_eq(lit("2022-01-01"))
                .and(col("date").lt_eq(lit("2022-12-31"))),
        )
        .collect()?;
    println!("Filtered data for 2022");
    Ok(df_filtered)
}

pub fn total_cases_by_country(df: &DataFrame) -> Result<DataFrame, Box<dyn Error>> {
    let df_total = df
        .lazy()
        .groupby([col("location")])
        .agg([col("new_cases").sum().alias("total_cases")])
        .sort("total_cases", SortOptions {
            descending: true,
            ..Default::default()
        })
        .collect()?;
    println!("Aggregated total cases by country");
    Ok(df_total)
}
