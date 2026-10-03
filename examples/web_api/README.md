# Web API Example

A Flask REST API demonstrating web framework migration from Python to Rust.

## What It Does

A simple CRUD API for managing items (inventory system):
- Create, Read, Update, Delete operations
- SQLite database backend
- JSON request/response format
- Proper error handling and HTTP status codes

## Files

- `app.py` — Flask application with REST endpoints
- `test_api.sh` — Bash script to test all endpoints
- `README.md` — This file

## Setup

```bash
# Install dependencies
pip install flask

# Run the server
python app.py
```

Server starts at `http://localhost:5000`

## API Endpoints

### List all items
```bash
GET /api/items
Response: {"success": true, "items": [...], "count": 5}
```

### Get specific item
```bash
GET /api/items/1
Response: {"success": true, "item": {...}}
```

### Create item
```bash
POST /api/items
Body: {"name": "Laptop", "description": "Dell XPS 15", "price": 1299.99}
Response: {"success": true, "item": {...}}
```

### Update item
```bash
PUT /api/items/1
Body: {"price": 1199.99}
Response: {"success": true, "item": {...}}
```

### Delete item
```bash
DELETE /api/items/1
Response: {"success": true, "message": "Item 1 deleted"}
```

### Health check
```bash
GET /api/health
Response: {"success": true, "status": "healthy", "timestamp": "..."}
```

## Testing

Use the provided test script:

```bash
chmod +x test_api.sh
./test_api.sh
```

Or use curl manually:

```bash
# Create an item
curl -X POST http://localhost:5000/api/items \
  -H "Content-Type: application/json" \
  -d '{"name": "Laptop", "description": "Dell XPS", "price": 1299.99}'

# List all items
curl http://localhost:5000/api/items

# Get specific item
curl http://localhost:5000/api/items/1

# Update item
curl -X PUT http://localhost:5000/api/items/1 \
  -H "Content-Type: application/json" \
  -d '{"price": 1199.99}'

# Delete item
curl -X DELETE http://localhost:5000/api/items/1
```

## Migration Notes

When migrating to Rust, expect these equivalents:

- `Flask` → `axum` or `actix-web` (async web framework)
- `sqlite3` → `sqlx` with SQLite feature (async database)
- `@app.route()` → `Router::new().route()` (routing)
- `request.get_json()` → `Json<T>` extractor (JSON parsing)
- `jsonify()` → `Json(value)` (JSON response)
- HTTP status codes → `StatusCode::OK`, `StatusCode::NOT_FOUND`, etc.
- Database connections → Connection pooling with `sqlx::SqlitePool`

## Expected Rust Structure

```rust
// main.rs
use axum::{
    routing::{get, post, put, delete},
    Router, Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};

#[derive(Serialize, Deserialize, FromRow)]
struct Item {
    id: i64,
    name: String,
    description: Option<String>,
    price: f64,
    created_at: String,
}

#[derive(Deserialize)]
struct CreateItemRequest {
    name: String,
    description: Option<String>,
    price: f64,
}

async fn list_items(State(pool): State<SqlitePool>) -> Result<Json<Response>, StatusCode> {
    // Implementation
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite:items.db").await.unwrap();
    
    let app = Router::new()
        .route("/api/items", get(list_items).post(create_item))
        .route("/api/items/:id", get(get_item).put(update_item).delete(delete_item))
        .route("/api/health", get(health_check))
        .with_state(pool);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## Testing Migration

Use this migration instruction in py2rs:

```markdown
Migrate this Flask REST API to Rust using axum web framework:

1. Use `axum` for HTTP routing and handlers
2. Use `sqlx` with SQLite for database operations (async)
3. Use `serde` for JSON (de)serialization
4. Use `tokio` as the async runtime
5. Preserve all endpoints with the same paths and methods
6. Keep the same JSON request/response formats
7. Use proper Rust error handling with `Result` and `?`
8. Use `anyhow` or `thiserror` for custom error types
9. Add connection pooling with `SqlitePool`
10. Use extractors: `Json<T>`, `Path<u64>`, `State<SqlitePool>`
11. Return proper HTTP status codes: `StatusCode::OK`, `NOT_FOUND`, `BAD_REQUEST`, etc.
12. Add #[derive(Serialize, Deserialize)] for all request/response structs
13. Initialize database schema on startup
14. Listen on 0.0.0.0:3000 (or make it configurable)

Dependencies to add to Cargo.toml:
- axum
- tokio (with "full" features)
- sqlx (with "runtime-tokio", "sqlite" features)
- serde (with "derive")
- serde_json
- anyhow
- chrono
```
