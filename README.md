# COVID-19 Analysis with Polars (Rust)

This project downloads COVID-19 data from Our World in Data, filters data for 2022, aggregates total cases per country using Polars, and saves the result as a CSV.

## Folder Structure

- `data/raw/` – stores downloaded CSVs  
- `data/processed/` – stores processed CSVs  
- `src/` – source code  

## Run

```bash
cargo run
```

The results will be saved in data/processed/total_cases_2022.csv.

---

✅ **How to Run**

1. Make sure Rust is installed (`rustc --version`)  
2. Create the folder structure: `data/raw` and `data/processed`  
3. Run:

```bash
cargo run
```
