#!/usr/bin/env python3
"""
Data analysis script demonstrating pandas/numpy migration.

This script loads a CSV dataset, performs analysis, and generates visualizations.
"""

import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path
import json


def load_data(filepath: str) -> pd.DataFrame:
    """Load CSV data into DataFrame."""
    df = pd.read_csv(filepath)
    print(f"Loaded {len(df)} rows from {filepath}")
    return df


def basic_statistics(df: pd.DataFrame) -> dict:
    """Calculate basic statistics."""
    stats = {
        'total_rows': len(df),
        'total_sales': float(df['sales'].sum()),
        'average_sales': float(df['sales'].mean()),
        'median_sales': float(df['sales'].median()),
        'std_sales': float(df['sales'].std()),
        'min_sales': float(df['sales'].min()),
        'max_sales': float(df['sales'].max()),
    }

    print("\n=== Basic Statistics ===")
    for key, value in stats.items():
        print(f"{key}: {value:.2f}")

    return stats


def sales_by_category(df: pd.DataFrame) -> pd.DataFrame:
    """Calculate total sales by category."""
    category_sales = df.groupby('category')['sales'].agg(['sum', 'mean', 'count'])
    category_sales = category_sales.sort_values('sum', ascending=False)

    print("\n=== Sales by Category ===")
    print(category_sales)

    return category_sales


def sales_by_month(df: pd.DataFrame) -> pd.DataFrame:
    """Calculate monthly sales trends."""
    df['date'] = pd.to_datetime(df['date'])
    df['month'] = df['date'].dt.to_period('M')

    monthly_sales = df.groupby('month')['sales'].sum()

    print("\n=== Monthly Sales ===")
    print(monthly_sales)

    return monthly_sales


def top_products(df: pd.DataFrame, n: int = 10) -> pd.DataFrame:
    """Find top N products by sales."""
    top_n = df.nlargest(n, 'sales')[['product', 'category', 'sales']]

    print(f"\n=== Top {n} Products ===")
    print(top_n.to_string(index=False))

    return top_n


def calculate_percentiles(df: pd.DataFrame) -> dict:
    """Calculate sales percentiles."""
    percentiles = [10, 25, 50, 75, 90, 95, 99]
    values = {f'p{p}': float(np.percentile(df['sales'], p)) for p in percentiles}

    print("\n=== Sales Percentiles ===")
    for p, v in values.items():
        print(f"{p}: {v:.2f}")

    return values


def plot_category_sales(category_sales: pd.DataFrame, output_path: str):
    """Create bar chart of sales by category."""
    plt.figure(figsize=(10, 6))
    category_sales['sum'].plot(kind='bar', color='steelblue')
    plt.title('Total Sales by Category', fontsize=14, fontweight='bold')
    plt.xlabel('Category', fontsize=12)
    plt.ylabel('Total Sales ($)', fontsize=12)
    plt.xticks(rotation=45, ha='right')
    plt.tight_layout()
    plt.savefig(output_path, dpi=300)
    print(f"\nSaved chart to {output_path}")
    plt.close()


def plot_monthly_trend(monthly_sales: pd.Series, output_path: str):
    """Create line chart of monthly sales trend."""
    plt.figure(figsize=(12, 6))
    monthly_sales.plot(kind='line', marker='o', color='darkgreen', linewidth=2)
    plt.title('Monthly Sales Trend', fontsize=14, fontweight='bold')
    plt.xlabel('Month', fontsize=12)
    plt.ylabel('Sales ($)', fontsize=12)
    plt.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(output_path, dpi=300)
    print(f"Saved chart to {output_path}")
    plt.close()


def plot_sales_distribution(df: pd.DataFrame, output_path: str):
    """Create histogram of sales distribution."""
    plt.figure(figsize=(10, 6))
    plt.hist(df['sales'], bins=50, color='coral', edgecolor='black', alpha=0.7)
    plt.title('Sales Distribution', fontsize=14, fontweight='bold')
    plt.xlabel('Sales Amount ($)', fontsize=12)
    plt.ylabel('Frequency', fontsize=12)
    plt.axvline(df['sales'].mean(), color='red', linestyle='--', linewidth=2, label='Mean')
    plt.axvline(df['sales'].median(), color='blue', linestyle='--', linewidth=2, label='Median')
    plt.legend()
    plt.tight_layout()
    plt.savefig(output_path, dpi=300)
    print(f"Saved chart to {output_path}")
    plt.close()


def save_report(stats: dict, output_path: str):
    """Save analysis report to JSON."""
    with open(output_path, 'w') as f:
        json.dump(stats, f, indent=2)
    print(f"\nSaved report to {output_path}")


def main():
    """Main analysis pipeline."""
    # Load data
    df = load_data('sales_data.csv')

    # Perform analysis
    stats = basic_statistics(df)
    category_sales = sales_by_category(df)
    monthly_sales = sales_by_month(df)
    top_10 = top_products(df, n=10)
    percentiles = calculate_percentiles(df)

    # Create visualizations
    output_dir = Path('output')
    output_dir.mkdir(exist_ok=True)

    plot_category_sales(category_sales, str(output_dir / 'category_sales.png'))
    plot_monthly_trend(monthly_sales, str(output_dir / 'monthly_trend.png'))
    plot_sales_distribution(df, str(output_dir / 'sales_distribution.png'))

    # Combine results
    report = {
        'basic_statistics': stats,
        'percentiles': percentiles,
        'top_products': top_10.to_dict('records'),
        'category_totals': category_sales['sum'].to_dict(),
        'monthly_totals': {str(k): float(v) for k, v in monthly_sales.items()}
    }

    save_report(report, str(output_dir / 'analysis_report.json'))

    print("\n=== Analysis Complete ===")
    print(f"Results saved to {output_dir}/")


if __name__ == '__main__':
    main()
