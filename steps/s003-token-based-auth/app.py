from flask import Flask, request, jsonify
from flask_cors import CORS
from werkzeug.security import generate_password_hash, check_password_hash
import uuid

app = Flask(__name__)
CORS(app)

# In-memory storage for testing
users = {
    "alice": generate_password_hash("password123"),
    "bob": generate_password_hash("hunter2")
}
tokens = {}  # token -> username
bookmarks = {}

# Login endpoint to get token
@app.route("/api/login", methods=["POST"])
def login():
    data = request.get_json()
    username = data.get("username")
    password = data.get("password")

    if username not in users or not check_password_hash(users[username], password):
        return jsonify({"error": "Invalid credentials"}), 401

    token = str(uuid.uuid4())  # generate simple token for demo
    tokens[token] = username
    if username not in bookmarks:
        bookmarks[username] = []

    return jsonify({"token": token})

# Token-auth protected route
def require_token():
    auth_header = request.headers.get("Authorization", "")
    if not auth_header.startswith("Bearer "):
        return None
    token = auth_header.split(" ", 1)[1]
    return tokens.get(token)

@app.route("/api/bookmark", methods=["POST"])
def bookmark():
    user = require_token()
    if not user:
        return jsonify({"error": "Unauthorized"}), 401

    data = request.get_json()
    url = data.get("url")
    if not url:
        return jsonify({"error": "Missing URL"}), 400

    bookmarks[user].append(url)
    return jsonify({"message": "Bookmark saved"}), 200

@app.route("/api/bookmarks", methods=["GET"])
def get_bookmarks():
    user = require_token()
    if not user:
        return jsonify({"error": "Unauthorized"}), 401
    return jsonify({"bookmarks": bookmarks[user]})

if __name__ == "__main__":
    app.run(debug=True)
