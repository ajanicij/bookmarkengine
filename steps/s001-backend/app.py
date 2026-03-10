from flask import Flask, request, jsonify
from functools import wraps
from werkzeug.security import check_password_hash, generate_password_hash
import base64

app = Flask(__name__)

# In-memory users and bookmarks (for testing only)
users = {
    "alice": generate_password_hash("password123"),
    "bob": generate_password_hash("hunter2")
}

bookmarks = {
    "alice": [],
    "bob": []
}

# Basic Auth decorator
def require_auth(f):
    @wraps(f)
    def decorated(*args, **kwargs):
        auth = request.headers.get("Authorization")
        if not auth or not auth.startswith("Basic "):
            return jsonify({"error": "Missing or invalid Authorization header"}), 401

        encoded_credentials = auth.split(" ", 1)[1]
        try:
            decoded = base64.b64decode(encoded_credentials).decode("utf-8")
            username, password = decoded.split(":", 1)
        except Exception:
            return jsonify({"error": "Invalid auth encoding"}), 400

        if username not in users or not check_password_hash(users[username], password):
            return jsonify({"error": "Invalid credentials"}), 403

        request.user = username  # store for use in endpoint
        return f(*args, **kwargs)
    return decorated

# Save a bookmark
@app.route("/api/bookmark", methods=["POST"])
@require_auth
def save_bookmark():
    data = request.get_json()
    url = data.get("url")
    if not url:
        return jsonify({"error": "Missing URL"}), 400

    user = request.user
    bookmarks[user].append(url)
    return jsonify({"message": "Bookmark saved"}), 200

# Retrieve all bookmarks for a user
@app.route("/api/bookmarks", methods=["GET"])
@require_auth
def get_bookmarks():
    user = request.user
    return jsonify({
        "user": user,
        "bookmarks": bookmarks[user]
    })

# Root test route
@app.route("/")
def index():
    return "Bookmark API Server is running!"

if __name__ == "__main__":
    app.run(debug=True)
