// Polyfill for Chrome/Firefox compatibility
if (typeof browser === "undefined") {
    var browser = chrome;
  }
  
  document.addEventListener("DOMContentLoaded", () => {
    const usernameInput = document.getElementById("username");
    const passwordInput = document.getElementById("password");
    const saveBtn = document.getElementById("save");
    const sendBtn = document.getElementById("send");
    const statusEl = document.getElementById("status");
  
    // Load saved credentials
    browser.storage.local.get(["username", "password"]).then((result) => {
      if (result.username) usernameInput.value = result.username;
      if (result.password) passwordInput.value = result.password;
    });
  
    // Save credentials
    saveBtn.addEventListener("click", () => {
      const username = usernameInput.value.trim();
      const password = passwordInput.value.trim();
  
      if (!username || !password) {
        updateStatus("Please enter both username and password.");
        return;
      }
  
      browser.storage.local.set({ username, password }).then(() => {
        updateStatus("Credentials saved.");
      });
    });
  
    // Send current tab URL
    sendBtn.addEventListener("click", async () => {
      const { username, password } = await browser.storage.local.get(["username", "password"]);
      if (!username || !password) {
        updateStatus("Please save your credentials first.");
        return;
      }
  
      const authHeader = "Basic " + btoa(`${username}:${password}`);
  
      try {
        const tabs = await browser.tabs.query({ active: true, currentWindow: true });
        const currentTab = tabs[0];
  
        if (!currentTab || !currentTab.url) {
          updateStatus("Could not get current tab.");
          return;
        }
  
        const response = await fetch("http://localhost:5000/api/bookmark", {
          method: "POST",
          headers: {
            "Authorization": authHeader,
            "Content-Type": "application/json"
          },
          body: JSON.stringify({ url: currentTab.url })
        });
  
        const responseText = await response.text();
  
        if (response.ok) {
          updateStatus("Bookmark saved!");
        } else {
          updateStatus(`Failed (${response.status}): ${responseText}`);
        }
      } catch (err) {
        console.error("Error sending request:", err);
        updateStatus("Network error. Check console.");
      }
    });
  
    function updateStatus(msg) {
      statusEl.textContent = msg;
      console.log(msg);
    }
  });
  