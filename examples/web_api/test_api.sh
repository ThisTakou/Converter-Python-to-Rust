#!/bin/bash

# Test script for Flask API

BASE_URL="http://localhost:5000"

echo "=== Testing Flask REST API ==="
echo ""

echo "1. Health check..."
curl -s "$BASE_URL/api/health" | python -m json.tool
echo -e "\n"

echo "2. Creating first item (Laptop)..."
curl -s -X POST "$BASE_URL/api/items" \
  -H "Content-Type: application/json" \
  -d '{"name": "Laptop", "description": "Dell XPS 15", "price": 1299.99}' | python -m json.tool
echo -e "\n"

echo "3. Creating second item (Mouse)..."
curl -s -X POST "$BASE_URL/api/items" \
  -H "Content-Type: application/json" \
  -d '{"name": "Mouse", "description": "Logitech MX Master", "price": 99.99}' | python -m json.tool
echo -e "\n"

echo "4. Creating third item (Keyboard)..."
curl -s -X POST "$BASE_URL/api/items" \
  -H "Content-Type: application/json" \
  -d '{"name": "Keyboard", "description": "Mechanical keyboard", "price": 149.99}' | python -m json.tool
echo -e "\n"

echo "5. List all items..."
curl -s "$BASE_URL/api/items" | python -m json.tool
echo -e "\n"

echo "6. Get specific item (id=1)..."
curl -s "$BASE_URL/api/items/1" | python -m json.tool
echo -e "\n"

echo "7. Update item price (id=1)..."
curl -s -X PUT "$BASE_URL/api/items/1" \
  -H "Content-Type: application/json" \
  -d '{"price": 1199.99}' | python -m json.tool
echo -e "\n"

echo "8. Update item description (id=2)..."
curl -s -X PUT "$BASE_URL/api/items/2" \
  -H "Content-Type: application/json" \
  -d '{"description": "Wireless mouse with USB-C"}' | python -m json.tool
echo -e "\n"

echo "9. Delete item (id=3)..."
curl -s -X DELETE "$BASE_URL/api/items/3" | python -m json.tool
echo -e "\n"

echo "10. List remaining items..."
curl -s "$BASE_URL/api/items" | python -m json.tool
echo -e "\n"

echo "11. Try to get deleted item (should return 404)..."
curl -s "$BASE_URL/api/items/3" | python -m json.tool
echo -e "\n"

echo "12. Try to create item with missing fields (should return 400)..."
curl -s -X POST "$BASE_URL/api/items" \
  -H "Content-Type: application/json" \
  -d '{"name": "Incomplete"}' | python -m json.tool
echo -e "\n"

echo "13. Try to create item with negative price (should return 400)..."
curl -s -X POST "$BASE_URL/api/items" \
  -H "Content-Type: application/json" \
  -d '{"name": "Invalid", "price": -10}' | python -m json.tool
echo -e "\n"

echo "=== Test complete ==="
