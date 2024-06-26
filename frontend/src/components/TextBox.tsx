import { useEffect, useState } from "react";
import Button from "./Button";

function TextBox({ socket }) {
  const [currentText, setCurrentText] = useState("");
  const [resultText, setResultText ] = useState("");

  useEffect(() => {
    socket.on("receiveText", (data) => {
      setResultText(data);
    });
  }, []);

  const sendTextFunction = () => {
    if (currentText !== "") {
      const textData = {
        text: currentText,
        time:
          new Date(Date.now()).getHours() +
          ":" +
          new Date(Date.now()).getMinutes(),
      };
      socket.emit("sendText", textData);
      setResultText(textData.text);
    }
  };
  
  return (
    <>
      <div>
        <p>{resultText}</p>
      </div>
      <div>
        <input
          type="text"
          placeholder="Enter Math Expression (RPN format)"
          onChange={(e) => {
            setCurrentText(e.target.value);
          }}
        />
      <Button color="primary" onClick={sendTextFunction}>
          &#9658;
      </Button>
      </div>
    </>
  );
}

export default TextBox;
