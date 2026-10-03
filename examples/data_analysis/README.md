# Data Analysis Example

A pandas/numpy data analysis script demonstrating scientific computing library migration.

## What It Does

This script performs sales data analysis:
1. Loads CSV data into a DataFrame
2. Calculates basic statistics (mean, median, std, etc.)
3. Aggregates sales by category and month
4. Finds top products
5. Calculates percentiles
6. Generates visualizations (bar charts, line plots, histograms)
7. Saves JSON report

## Files

- `analyze.py` — Main analysis script
- `generate_data.py` — Generate sample sales data
- `sales_data.csv` — Sample dataset (generated)
- `README.md` — This file

## Setup

```bash
# Install dependencies
pip install pandas numpy matplotlib

# Generate sample data
python generate_data.py

# Run analysis
python analyze.py
```

## Output

Results are saved to `output/` directory:
- `category_sales.png` — Bar chart of sales by category
- `monthly_trend.png` — Line chart of monthly sales
- `sales_distribution.png` — Histogram with mean/median lines
- `analysis_report.json` — Comprehensive JSON report

## Migration Notes

When migrating to Rust, expect these equivalents:

### Data Structures
- `pandas.DataFrame` → `polars::DataFrame` (similar API, faster)
- `pandas.Series` → `polars::Series`
- `numpy.ndarray` → `ndarray::Array` or `Vec<f64>`

### Operations
- `df.read_csv()` → `CsvReader::from_path().finish()?`
- `df.groupby().agg()` → `df.groupby()?.agg()`
- `df.sort_values()` → `df.sort()`
- `df.nlargest()` → `df.sort().head()`
- `np.percentile()` → Manual calculation or use `statrs`
- `pd.to_datetime()` → `chrono::NaiveDate::parse_from_str()`

### Visualization
- `matplotlib.pyplot` → `plotters` (create PNG/SVG charts)
- `plt.figure()` → `BitMapBackend::new()`
- `plt.plot()` → `ChartBuilder::on()` with `LineSeries`
- `plt.hist()` → `Histogram` series
- `plt.bar()` → `Rectangle` series

### File I/O
- `json.dump()` → `serde_json::to_writer_pretty()`
- `Path.mkdir()` → `std::fs::create_dir_all()`

## Expected Rust Structure

```rust
// analyze.rs
use polars::prelude::*;
use plotters::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize)]
struct Statistics {
    total_rows: usize,
    total_sales: f64,
    average_sales: f64,
    median_sales: f64,
    std_sales: f64,
    min_sales: f64,
    max_sales: f64,
}

fn load_data(filepath: &str) -> Result<DataFrame> {
    CsvReader::from_path(filepath)?
        .has_header(true)
        .finish()
}

fn basic_statistics(df: &DataFrame) -> Result<Statistics> {
    let sales = df.column("sales")?.f64()?;
    
    Ok(Statistics {
        total_rows: df.height(),
        total_sales: sales.sum().unwrap_or(0.0),
        average_sales: sales.mean().unwrap_or(0.0),
        median_sales: sales.median().unwrap_or(0.0),
        std_sales: sales.std(1).unwrap_or(0.0),
        min_sales: sales.min().unwrap_or(0.0),
        max_sales: sales.max().unwrap_or(0.0),
    })
}

fn sales_by_category(df: &DataFrame) -> Result<DataFrame> {
    df.groupby(["category"])?
        .agg(&[
            ("sales", &["sum", "mean", "count"]),
        ])?
        .sort(["sales_sum"], true)
}

fn plot_category_sales(df: &DataFrame, output_path: &str) -> Result<()> {
    let root = BitMapBackend::new(output_path, (1000, 600))
        .into_drawing_area();
    root.fill(&WHITE)?;
    
    // Build chart with plotters
    let mut chart = ChartBuilder::on(&root)
        .caption("Total Sales by Category", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0..10, 0.0..100000.0)?;
    
    // Draw bars...
    
    Ok(())
}

fn main() -> Result<()> {
    let df = load_data("sales_data.csv")?;
    let stats = basic_statistics(&df)?;
    
    println!("Basic Statistics: {:#?}", stats);
    
    let category_sales = sales_by_category(&df)?;
    plot_category_sales(&category_sales, "output/category_sales.png")?;
    
    // Save report
    fs::create_dir_all("output")?;
    let report = serde_json::to_string_pretty(&stats)?;
    fs::write("output/analysis_report.json", report)?;
    
    Ok(())
}
```

## Testing Migration

Use this migration instruction in py2rs:

```markdown
Migrate this pandas/numpy data analysis script to Rust:

1. Use `polars` for DataFrame operations (faster and more idiomatic than pandas bindings)
2. Use `ndarray` for numerical arrays if needed (or just Vec<f64>)
3. Use `plotters` for creating PNG charts
4. Use `chrono` for date/time parsing and manipulation
5. Use `serde` and `serde_json` for JSON serialization
6. Use `statrs` for statistical functions (percentiles, distributions)

Key transformations:
- DataFrame operations: groupby, agg, sort, filter → polars API
- Date parsing: pd.to_datetime() → chrono::NaiveDate
- Plotting: matplotlib → plotters (BitMapBackend, ChartBuilder)
- Statistics: numpy functions → manual calculation or statrs
- File I/O: Path, json → std::fs, serde_json

Preserve:
- Same analysis logic and calculations
- Same output file structure (PNG charts + JSON report)
- Same statistics and aggregations

Dependencies for Cargo.toml:
- polars (with "lazy", "csv-file" features)
- plotters (with "bitmap" feature)
- serde (with "derive")
- serde_json
- chrono
- statrs
- anyhow (for error handling)

Note: polars uses lazy evaluation, so add .collect()? after lazy operations.
Use proper error handling with Result<T, anyhow::Error> and the ? operator.
```

## Additional Notes

**Performance**: Rust version will be significantly faster due to:
- polars' optimized query engine
- No Python interpreter overhead
- Better memory layout and cache utilization
- Parallel execution by default

**Type Safety**: Rust version benefits from compile-time checking:
- Column types verified at compile time (if using typed operations)
- No runtime type errors
- Explicit error handling

**Deployment**: Compiled binary has no dependencies (unlike Python + pandas + numpy).
