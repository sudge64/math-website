import { useEffect, useState } from "react";
import Button from "./Button";

function TextBox({ socket, currentText, setCurrentText, handleSendText }) {
  const [resultText, setResultText] = useState("");

  useEffect(() => {
    // Listen for incoming messages from the server
    socket.on("receiveText", (data) => {
      setResultText(data);
    });

    // Add a keydown event listener to handle keyboard input
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Enter") {
        // Handle equal sign or enter for submit
        handleSendText()
      }
    };

    // Attach the event listener
    window.addEventListener("keydown", handleKeyDown);

    // Cleanup event listener on component unmount
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [socket, setCurrentText, handleSendText]);

  return (
    <>
      <div>
        <p>{resultText}</p>
      </div>
      <div>
        <input
          type="text"
          value={currentText}
          placeholder="Enter Math Expression"
          onChange={(e) => {
            setCurrentText(e.target.value);
          }}
        />
        <Button color="primary" onClick={handleSendText}>
          &#9658;
        </Button>
      </div>
    </>
  );
}

export default TextBox;
