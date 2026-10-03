#!/usr/bin/env python3
"""
Simple Python script demonstrating common patterns for migration testing.

This script reads a JSON configuration file, processes text data,
and writes results to an output file.
"""

import json
import sys
import argparse
from pathlib import Path
from datetime import datetime


def load_config(config_path: str) -> dict:
    """Load configuration from JSON file."""
    with open(config_path, 'r') as f:
        return json.load(f)


def process_text(text: str, uppercase: bool = False) -> str:
    """Process text with optional transformation."""
    # Remove extra whitespace
    text = ' '.join(text.split())

    # Apply transformation
    if uppercase:
        text = text.upper()

    return text


def count_words(text: str) -> dict:
    """Count word frequencies in text."""
    words = text.lower().split()
    counts = {}

    for word in words:
        # Remove punctuation
        word = word.strip('.,!?;:')
        if word:
            counts[word] = counts.get(word, 0) + 1

    return counts


def save_results(output_path: str, results: dict):
    """Save processing results to JSON file."""
    results['timestamp'] = datetime.now().isoformat()

    with open(output_path, 'w') as f:
        json.dump(results, f, indent=2)

    print(f"Results saved to {output_path}")


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description='Process text data with configurable options'
    )
    parser.add_argument('input', help='Input text file')
    parser.add_argument('-c', '--config', default='config.json',
                       help='Configuration file (default: config.json)')
    parser.add_argument('-o', '--output', default='results.json',
                       help='Output file (default: results.json)')
    parser.add_argument('--uppercase', action='store_true',
                       help='Convert text to uppercase')

    args = parser.parse_args()

    # Load configuration
    try:
        config = load_config(args.config)
        print(f"Loaded config: {config}")
    except FileNotFoundError:
        print(f"Warning: Config file {args.config} not found, using defaults")
        config = {'enabled': True}

    # Read input file
    input_path = Path(args.input)
    if not input_path.exists():
        print(f"Error: Input file {args.input} does not exist", file=sys.stderr)
        sys.exit(1)

    text = input_path.read_text()

    # Process text
    processed_text = process_text(text, uppercase=args.uppercase)

    # Count words
    word_counts = count_words(processed_text)

    # Prepare results
    results = {
        'input_file': str(input_path),
        'processed_text': processed_text,
        'word_count': len(processed_text.split()),
        'unique_words': len(word_counts),
        'top_words': sorted(word_counts.items(), key=lambda x: x[1], reverse=True)[:10],
        'config': config
    }

    # Save results
    save_results(args.output, results)

    print(f"Processed {results['word_count']} words ({results['unique_words']} unique)")


if __name__ == '__main__':
    main()
