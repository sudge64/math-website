const childProcess = require("./childProcess");
const express = require("express");
const cors = require("cors");
const app = express();
const http = require("http").createServer(app)
const io = require("socket.io")(http);

const port = 3001;

async function childProc() {
  return await childProcess.childProcess();
}

app.use(express.json());
app.use(cors({ 
  origin: "http://localhost:5173",
  method: ['GET', 'POST'],
  allowedHeaders: ['Content-Type']
}));

app.get("/", (req, res) => {
  res.send("Hello from backend");
});

app.post("/button", async (req, res) => {
  try {
    const result = await childProc();
    res.json({ message: "Button clicked", result });
  } catch (error) {
    res.status(500).json({ message: "Error occurred", error });
  }
});

http.listen(port, () => {
  console.log(`Running on ${port}`);
});

io.on('connection', (socket) => {
    console.log('new client connected');
});
