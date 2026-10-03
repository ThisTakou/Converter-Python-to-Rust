# Example Python Projects

This directory contains demo Python projects for testing py2rs migration capabilities.

## Available Examples

### 1. simple_script/
A basic Python script demonstrating common patterns:
- File I/O operations
- String manipulation
- Command-line arguments
- JSON processing

**Good for:** Testing basic syntax migration and standard library equivalents.

### 2. web_api/
A Flask-based REST API with:
- HTTP endpoints (GET, POST)
- JSON request/response
- Database operations (SQLite)
- Error handling

**Good for:** Testing web framework migration (Flask → axum/actix-web).

### 3. data_analysis/
A data science project with:
- pandas DataFrame operations
- numpy array computations
- matplotlib plotting
- CSV file processing

**Good for:** Testing scientific computing library migration.

## How to Use

1. Open py2rs application
2. Click "Выбрать проект" / "Select Project"
3. Navigate to one of the example directories
4. Configure your LLM provider
5. Add migration instructions in the documentation field
6. Start migration

## Testing Recommendations

**For simple_script:**
```markdown
Migrate this Python script to idiomatic Rust. Use std::fs for file operations, 
clap for CLI arguments, and serde_json for JSON. Keep the same command-line interface.
```

**For web_api:**
```markdown
Migrate this Flask API to axum. Use sqlx with SQLite for database operations.
Preserve all endpoints and the same JSON request/response formats.
Add proper error handling with Result types.
```

**For data_analysis:**
```markdown
Migrate this data analysis script to Rust using polars for DataFrames,
ndarray for numerical operations, and plotters for visualization.
Maintain the same output format.
```

## Expected Challenges

Each example is designed to test specific migration challenges:

- **simple_script** — Basic syntax, error handling patterns
- **web_api** — Async/await translation, framework differences
- **data_analysis** — Library API differences, performance optimization

## Contributing Examples

Want to add more examples? Submit a PR with:
- Clear, well-commented Python code
- README explaining what the code does
- Suggested migration instructions
- Expected Rust crate equivalents
