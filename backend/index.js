const childProcess = require("./childProcess");
const express = require("express");
const app = express();
const http = require("http");
const cors = require("cors");
const { Server } = require("socket.io");
const { ChildProcess } = require("child_process");
app.use(cors());

const server = http.createServer(app);

const io = new Server(server, {
  cors: {
    origin: "http://localhost:5173",
    methods: ["GET", "POST"],
  },
});

const port = process.env.PORT || 3001;

async function childProc(stringyBoi) {
  return await childProcess.childProcess(stringyBoi);
}

app.get("/", (req, res) => {
  res.send("Hello from backend");
});

app.post("/button", async (req, res) => {
  try {
    const result = await childProc("1 2 -");
    res.json({ message: "Button clicked", result });
    console.log(`result: ${result}`)
  } catch (error) {
    res.status(500).json({ message: "Error occurred", error });
  }
});

io.on("connection", (socket) => {
  console.log(`new client connected: ${socket.id}`);

  socket.on("disconnect", () => {
    console.log(`client disconnected: ${socket.id}`);
  });

  socket.on("sendText", async (data) => {
    console.log(data);
    try {
      const result = await childProc(data.text);
      console.log(`result: ${result}`);
      socket.emit("receiveText", result);
    } catch (e) {
      socket.emit("receiveText", e);
    }
  });
});

server.listen(port, () => {
  console.log("SERVER RUNNING");
});
