const util = require("util");
const { spawn } = require("node:child_process");

async function childProcess() {
  return new Promise((resolve, reject) => {
    const ls = spawn("ls", ["-al", "."]);
    let output = "";

    ls.stdout.on("data", (data) => {
      output += data.toString();
    });

    ls.stderr.on("data", (data) => {
      output += data.toString();
    });

    ls.on("close", (code) => {
      if (code !== 0) {
        return reject(`Child process exited with code ${code}`);
      }
      resolve(output);
    });

    ls.on("error", (err) => {
      reject(`Failed to start child process: ${err}`);
    });
  });
}
module.exports = { childProcess };
