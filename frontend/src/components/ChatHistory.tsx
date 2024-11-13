import React from "react";

interface ChatHistoryProps {
  history: { text: string; result: string }[];
}

const ChatHistory: React.FC<ChatHistoryProps> = ({ history }) => {
  return (
    <div className="chat-history">
      {history
        .slice()
        .reverse()
        .map((entry, index) => (
          <div key={index} className="chat-entry">
            <span className="chat-text">{entry.text}</span>
            <span className="chat-result">{entry.result}</span>
          </div>
        ))}
    </div>
  );
};

export default ChatHistory;
