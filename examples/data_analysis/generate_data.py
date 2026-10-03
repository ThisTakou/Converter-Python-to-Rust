#!/usr/bin/env python3
"""Generate sample sales data for testing."""

import pandas as pd
import numpy as np
from datetime import datetime, timedelta

# Set random seed for reproducibility
np.random.seed(42)

# Parameters
num_records = 1000
start_date = datetime(2023, 1, 1)
end_date = datetime(2023, 12, 31)

# Categories and products
categories = {
    'Electronics': ['Laptop', 'Smartphone', 'Tablet', 'Headphones', 'Monitor'],
    'Furniture': ['Desk', 'Chair', 'Bookshelf', 'Table', 'Lamp'],
    'Clothing': ['Shirt', 'Pants', 'Jacket', 'Shoes', 'Hat'],
    'Books': ['Fiction', 'Non-Fiction', 'Textbook', 'Magazine', 'Comic'],
    'Sports': ['Basketball', 'Soccer Ball', 'Tennis Racket', 'Yoga Mat', 'Dumbbells']
}

# Generate data
data = []

for _ in range(num_records):
    # Random date
    days_diff = (end_date - start_date).days
    random_days = np.random.randint(0, days_diff)
    date = start_date + timedelta(days=random_days)

    # Random category and product
    category = np.random.choice(list(categories.keys()))
    product = np.random.choice(categories[category])

    # Random sales amount (log-normal distribution for realistic sales data)
    sales = np.random.lognormal(mean=5, sigma=1.5)

    data.append({
        'date': date.strftime('%Y-%m-%d'),
        'category': category,
        'product': product,
        'sales': round(sales, 2)
    })

# Create DataFrame
df = pd.DataFrame(data)

# Sort by date
df = df.sort_values('date').reset_index(drop=True)

# Save to CSV
df.to_csv('sales_data.csv', index=False)

print(f"Generated {len(df)} records")
print("\nFirst few rows:")
print(df.head())
print("\nBasic statistics:")
print(df['sales'].describe())
print("\nRecords per category:")
print(df['category'].value_counts())
