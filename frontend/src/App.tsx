import io from "socket.io-client";
import { useEffect, useState } from "react";
import "./App.css";
import Button from "./components/Button";
import TextBox from "./components/TextBox";
import { MathJax, MathJaxContext } from "better-react-mathjax";

const socket = io.connect("http://localhost:3001");

function App() {
  const [message, setMessage] = useState("");
  const [response, setResponse] = useState("");
  const [currentText, setCurrentText] = useState("");

  useEffect(() => {
    fetch("http://localhost:3001/")
      .then((message) => message.text())
      .then((data) => setMessage(data));
  });

  const handleButtonClick = (char: string) => {
    setCurrentText((prev) => prev + char);
  };

  const handleBackspaceClick = () => {
    setCurrentText((prev) => prev.slice(0, -1));
  };

  const handleSendText = () => {
    if (currentText !== "") {
      const textData = {
        text: currentText,
        time:
          new Date(Date.now()).getHours() +
          ":" +
          new Date(Date.now()).getMinutes() +
          " " +
          new Date(Date.now()).getDay(),
      };
      socket.emit("sendText", textData);
      setResponse(textData.text);
    }
  };

  return (
    <>
      <MathJaxContext>
        <TextBox
          socket={socket}
          currentText={currentText}
          setCurrentText={setCurrentText}
          handleSendText={handleSendText}
        />
        <div className="grid-buttons">
          <Button color="primary" char="7" onClick={handleButtonClick}>
            <MathJax>{"$$7$$"}</MathJax>
          </Button>
          <Button color="primary" char="8" onClick={handleButtonClick}>
            <MathJax>{"$$8$$"}</MathJax>
          </Button>
          <Button color="primary" char="9" onClick={handleButtonClick}>
            <MathJax>{"$$9$$"}</MathJax>
          </Button>
          <Button color="primary" char="/" onClick={handleButtonClick}>
            <MathJax>{"$$/$$"}</MathJax>
          </Button>
          <Button color="primary" char="\pi" onClick={handleButtonClick}>
            <MathJax>{"$$\\pi$$"}</MathJax>
          </Button>
          <Button color="primary" char="4" onClick={handleButtonClick}>
            <MathJax>{"$$4$$"}</MathJax>
          </Button>
          <Button color="primary" char="5" onClick={handleButtonClick}>
            <MathJax>{"$$5$$"}</MathJax>
          </Button>
          <Button color="primary" char="6" onClick={handleButtonClick}>
            <MathJax>{"$$6$$"}</MathJax>
          </Button>
          <Button color="primary" char="*" onClick={handleButtonClick}>
            <MathJax>{"$$*$$"}</MathJax>
          </Button>
          <Button color="primary" char="\sqrt{}" onClick={handleButtonClick}>
            <MathJax>{"$$\\sqrt{}$$"}</MathJax>
          </Button>
          <Button color="primary" char="1" onClick={handleButtonClick}>
            <MathJax>{"$$1$$"}</MathJax>
          </Button>
          <Button color="primary" char="2" onClick={handleButtonClick}>
            <MathJax>{"$$2$$"}</MathJax>
          </Button>
          <Button color="primary" char="3" onClick={handleButtonClick}>
            <MathJax>{"$$3$$"}</MathJax>
          </Button>
          <Button color="primary" char="-" onClick={handleButtonClick}>
            <MathJax>{"$$-$$"}</MathJax>
          </Button>
          <Button color="primary" char="^" onClick={handleButtonClick}>
            <MathJax>{"$$x^{y}$$"}</MathJax>
          </Button>
          <Button color="primary" char="0" onClick={handleButtonClick}>
            <MathJax>{"$$0$$"}</MathJax>
          </Button>
          <Button color="primary" char="." onClick={handleButtonClick}>
            <MathJax>{"$$.$$"}</MathJax>
          </Button>
          <Button color="primary" char="%" onClick={handleButtonClick}>
            <MathJax>{"$$\\%$$"}</MathJax>
          </Button>
          <Button color="primary" char="+" onClick={handleButtonClick}>
            <MathJax>{"$$+$$"}</MathJax>
          </Button>
          <Button color="success" char="=" onClick={handleSendText}>
            <MathJax>{"$$=$$"}</MathJax>
          </Button>
          <Button color="primary" char=" " onClick={handleButtonClick}>
            ⎵
          </Button>
          <Button
            color="primary"
            char="backspace"
            onClick={handleBackspaceClick}
          >
            ⌫
          </Button>
        </div>
      </MathJaxContext>
    </>
  );
}

export default App;
