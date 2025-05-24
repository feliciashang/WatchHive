import React from 'react';

const App: React.FC = () => {
  const handleSkip = () => {
    chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
      const activeTabId = tabs[0]?.id;
      if (!activeTabId) return;

      chrome.scripting.executeScript({
        target: { tabId: activeTabId },
        func: (skipTo: number) => {
          const video = document.querySelector<HTMLVideoElement>('video');
          if (video) video.currentTime = skipTo;
        },
        args: [10],
      });
    });
  };

  return (
    <div>
      <button onClick={handleSkip}>Skip to 10 Seconds</button>
    </div>
  );
};

export default App;
