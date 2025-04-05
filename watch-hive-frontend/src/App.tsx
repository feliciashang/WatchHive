import './App.css'

async function App() {
  let [tab] = await chrome.tabs.query({ active: true});
  chrome.sidePanel.setOptions({
    tabId: tab.id,                // Set the current tab's ID
    path: 'sidepanel.html',        // Path to the HTML file for the side panel
    enabled: true                  // Enable the side panel
  });
}

export default App
