const express = require("express");
const cors = require("cors");

const app = express();
const port = 3001;

app.use(express.json());
app.use(cors({ origin: true }));

app.get('/', (req, res) => {
  res.send('Hello from backend');
});

app.listen(port, () => {
  console.log(`Running on ${port}`);
});
