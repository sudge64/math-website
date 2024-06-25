import io from "socket.io-client";
import { useEffect, useState } from "react";
import "./App.css";
import Button from "./components/Button";

const socket = io.connect("http://localhost:3001")

function App() {
  const [message, setMessage] = useState("");
  const [response, setResponse] = useState("");

  useEffect(() => {
    fetch("http://localhost:3001/")
      .then((message) => message.text())
      .then((data) => setMessage(data));
  });

  const handleButtonClick = async () => {
    try {
      const res = await fetch("http://localhost:3001/button", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
      });
      const data = await res.json();
      setResponse(data.result);
    } catch (error) {
      console.error("ERROR: ", error);
      setResponse(`ERROR: ${error}`);
    }
  };

  return (
    <>
      <Button color="primary" onClick={handleButtonClick}>
        Press Me
      </Button>
      <pre>{response}</pre>
      <p>{message}</p>
    </>
  );
}

export default App;
