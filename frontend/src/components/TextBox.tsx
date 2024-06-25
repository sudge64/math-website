import React, { useEffect, useState } from "react";
import Button from "./Button";

function TextBox({ socket }) {
  const [currentText, setCurrentText] = useState("");
  const [textList, setTextList] = useState([]);
  const sendText = async () => {
    if (currentText !== "") {
      const textData = {
        text: currentText,
        time:
          new Date(Date.now()).getHours() +
          ":" +
          new Date(Date.now()).getMinutes(),
      };
      await socket.emit("send_text", textData);
      setTextList((list) => [...list, textData]);
      setCurrentText("");
    }
  };

  useEffect(() => {
    socket.on("receive_text", (data) => {
      setTextList((list) => [...list, data]);
    });
  }, [socket]);
  
  return (
    <>
      <div>
        <input
          type="text"
          placeholder="Enter Math Expression"
          onChange={(event) => {
            setCurrentText(event.target.value);
          }}
        />
      <Button color="primary" onClick={sendText}>
          &#9658;
      </Button>
      </div>
    </>
  );
}

export default TextBox;
