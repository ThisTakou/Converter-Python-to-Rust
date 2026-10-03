#!/usr/bin/env python3
"""
Simple REST API with Flask demonstrating web framework migration.

Endpoints:
- GET /api/items - List all items
- GET /api/items/<id> - Get specific item
- POST /api/items - Create new item
- PUT /api/items/<id> - Update item
- DELETE /api/items/<id> - Delete item
"""

from flask import Flask, request, jsonify
import sqlite3
import json
from datetime import datetime
from pathlib import Path


app = Flask(__name__)
DB_PATH = 'items.db'


def get_db():
    """Get database connection."""
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    """Initialize database schema."""
    conn = get_db()
    conn.execute('''
        CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            price REAL NOT NULL,
            created_at TEXT NOT NULL
        )
    ''')
    conn.commit()
    conn.close()


@app.route('/api/items', methods=['GET'])
def list_items():
    """Get all items."""
    try:
        conn = get_db()
        cursor = conn.execute('SELECT * FROM items ORDER BY created_at DESC')
        items = [dict(row) for row in cursor.fetchall()]
        conn.close()

        return jsonify({
            'success': True,
            'items': items,
            'count': len(items)
        })
    except Exception as e:
        return jsonify({
            'success': False,
            'error': str(e)
        }), 500


@app.route('/api/items/<int:item_id>', methods=['GET'])
def get_item(item_id):
    """Get specific item by ID."""
    try:
        conn = get_db()
        cursor = conn.execute('SELECT * FROM items WHERE id = ?', (item_id,))
        row = cursor.fetchone()
        conn.close()

        if row is None:
            return jsonify({
                'success': False,
                'error': 'Item not found'
            }), 404

        return jsonify({
            'success': True,
            'item': dict(row)
        })
    except Exception as e:
        return jsonify({
            'success': False,
            'error': str(e)
        }), 500


@app.route('/api/items', methods=['POST'])
def create_item():
    """Create new item."""
    try:
        data = request.get_json()

        # Validate required fields
        if not data or 'name' not in data or 'price' not in data:
            return jsonify({
                'success': False,
                'error': 'Missing required fields: name, price'
            }), 400

        name = data['name']
        description = data.get('description', '')
        price = float(data['price'])

        if price < 0:
            return jsonify({
                'success': False,
                'error': 'Price must be non-negative'
            }), 400

        created_at = datetime.now().isoformat()

        conn = get_db()
        cursor = conn.execute(
            'INSERT INTO items (name, description, price, created_at) VALUES (?, ?, ?, ?)',
            (name, description, price, created_at)
        )
        item_id = cursor.lastrowid
        conn.commit()
        conn.close()

        return jsonify({
            'success': True,
            'item': {
                'id': item_id,
                'name': name,
                'description': description,
                'price': price,
                'created_at': created_at
            }
        }), 201
    except ValueError:
        return jsonify({
            'success': False,
            'error': 'Invalid price format'
        }), 400
    except Exception as e:
        return jsonify({
            'success': False,
            'error': str(e)
        }), 500


@app.route('/api/items/<int:item_id>', methods=['PUT'])
def update_item(item_id):
    """Update existing item."""
    try:
        data = request.get_json()

        if not data:
            return jsonify({
                'success': False,
                'error': 'No data provided'
            }), 400

        conn = get_db()

        # Check if item exists
        cursor = conn.execute('SELECT * FROM items WHERE id = ?', (item_id,))
        if cursor.fetchone() is None:
            conn.close()
            return jsonify({
                'success': False,
                'error': 'Item not found'
            }), 404

        # Build update query dynamically
        fields = []
        values = []

        if 'name' in data:
            fields.append('name = ?')
            values.append(data['name'])

        if 'description' in data:
            fields.append('description = ?')
            values.append(data['description'])

        if 'price' in data:
            price = float(data['price'])
            if price < 0:
                conn.close()
                return jsonify({
                    'success': False,
                    'error': 'Price must be non-negative'
                }), 400
            fields.append('price = ?')
            values.append(price)

        if not fields:
            conn.close()
            return jsonify({
                'success': False,
                'error': 'No fields to update'
            }), 400

        values.append(item_id)
        query = f"UPDATE items SET {', '.join(fields)} WHERE id = ?"

        conn.execute(query, values)
        conn.commit()

        # Fetch updated item
        cursor = conn.execute('SELECT * FROM items WHERE id = ?', (item_id,))
        updated_item = dict(cursor.fetchone())
        conn.close()

        return jsonify({
            'success': True,
            'item': updated_item
        })
    except ValueError:
        return jsonify({
            'success': False,
            'error': 'Invalid price format'
        }), 400
    except Exception as e:
        return jsonify({
            'success': False,
            'error': str(e)
        }), 500


@app.route('/api/items/<int:item_id>', methods=['DELETE'])
def delete_item(item_id):
    """Delete item."""
    try:
        conn = get_db()
        cursor = conn.execute('DELETE FROM items WHERE id = ?', (item_id,))
        deleted_count = cursor.rowcount
        conn.commit()
        conn.close()

        if deleted_count == 0:
            return jsonify({
                'success': False,
                'error': 'Item not found'
            }), 404

        return jsonify({
            'success': True,
            'message': f'Item {item_id} deleted'
        })
    except Exception as e:
        return jsonify({
            'success': False,
            'error': str(e)
        }), 500


@app.route('/api/health', methods=['GET'])
def health_check():
    """Health check endpoint."""
    return jsonify({
        'success': True,
        'status': 'healthy',
        'timestamp': datetime.now().isoformat()
    })


if __name__ == '__main__':
    init_db()
    print("Database initialized")
    print("Starting server on http://localhost:5000")
    print("\nAvailable endpoints:")
    print("  GET    /api/items")
    print("  GET    /api/items/<id>")
    print("  POST   /api/items")
    print("  PUT    /api/items/<id>")
    print("  DELETE /api/items/<id>")
    print("  GET    /api/health")

    app.run(debug=True, host='0.0.0.0', port=5000)
