if (typeof browser === "undefined") {
  var browser = chrome;
}

document.addEventListener("DOMContentLoaded", () => {
  const usernameInput = document.getElementById("username");
  const passwordInput = document.getElementById("password");
  const saveBtn = document.getElementById("save");
  const sendBtn = document.getElementById("send");
  const statusEl = document.getElementById("status");

  // Load saved token and username
  browser.storage.local.get(["username", "token"]).then((result) => {
    if (result.username) usernameInput.value = result.username;
  });

  // Login and get token
  saveBtn.addEventListener("click", async () => {
    const username = usernameInput.value.trim();
    const password = passwordInput.value.trim();
    if (!username || !password) return updateStatus("Missing credentials");

    try {
      const response = await fetch("http://localhost:5000/api/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username, password })
      });

      const data = await response.json();
      if (response.ok) {
        await browser.storage.local.set({ username, token: data.token });
        updateStatus("Logged in! Token saved.");
      } else {
        updateStatus(`Login failed: ${data.error}`);
      }
    } catch (err) {
      console.error(err);
      updateStatus("Network error");
    }
  });

  // Send current URL using token
  sendBtn.addEventListener("click", async () => {
    const { token } = await browser.storage.local.get("token");
    if (!token) return updateStatus("You must log in first.");

    try {
      const tabs = await browser.tabs.query({ active: true, currentWindow: true });
      const currentTab = tabs[0];
      const url = currentTab.url;

      const response = await fetch("http://localhost:5000/api/bookmark", {
        method: "POST",
        headers: {
          "Authorization": `Bearer ${token}`,
          "Content-Type": "application/json"
        },
        body: JSON.stringify({ url })
      });

      const text = await response.text();
      if (response.ok) {
        updateStatus("URL bookmarked!");
      } else {
        updateStatus(`Error ${response.status}: ${text}`);
      }
    } catch (err) {
      console.error(err);
      updateStatus("Failed to send URL");
    }
  });

  function updateStatus(msg) {
    statusEl.textContent = msg;
    console.log(msg);
  }
});
