#!/usr/bin/env bash

curl -u alice:password123  -H "Content-Type: application/json"  \
  -d '{"url": "https://example.com"}' \
  -X POST  http://127.0.0.1:5000/api/bookmark
