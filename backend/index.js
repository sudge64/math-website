const childProcess = require("./childProcess");
const express = require("express");
const app = express();
const http = require("http");
const cors = require("cors");
const { Server } = require("socket.io");
app.use(cors());

const server = http.createServer(app);

const io = new Server(server, {
  cors: {
    origin: "http://localhost:5173",
    methods: ["GET", "POST"],
  },
});

const port = 3001;

async function childProc() {
  return await childProcess.childProcess();
}

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

io.on("connection", (socket) => {
  console.log(`new client connected: ${socket.id}`);

  socket.on("disconnect", () => {
    console.log(`client disconnected: ${socket.id}`);
  });
});

server.listen(port, () => {
  console.log("SERVER RUNNING");
});
