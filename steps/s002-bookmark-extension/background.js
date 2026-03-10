if (typeof browser === "undefined") {
    var browser = chrome;
  }
  
  function isFirefox() {
    return typeof InstallTrigger !== 'undefined';
  }
  
  chrome.action.onClicked.addListener((tab) => {
    if (!tab.url) return;
  
    const urlToSend = tab.url;
    const userId = "example_user_id"; // Or retrieve from local storage / auth
  
    fetch("http://127.0.0.1:5000/api/bookmark", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        url: urlToSend,
        userId: userId
      })
    })
    .then(response => {
      if (response.ok) {
        console.log("URL saved successfully!");
      } else {
        console.error("Failed to save URL.");
      }
    })
    .catch(error => {
      console.error("Error:", error);
    });
  });
  