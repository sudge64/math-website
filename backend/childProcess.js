const util = require("util");
const { spawn } = require("node:child_process");

async function childProcess() {
  return new Promise((resolve, reject) => {
    // const command = spawn("ls", ["-al", "."]);
    const command = spawn("date");
    let output = "";

    command.stdout.on("data", (data) => {
      output += data.toString();
    });

    command.stderr.on("data", (data) => {
      output += data.toString();
    });

    command.on("close", (code) => {
      if (code !== 0) {
        return reject(`Child process exited with code ${code}`);
      }
      resolve(output);
    });

    command.on("error", (err) => {
      reject(`Failed to start child process: ${err}`);
    });
  });
}
module.exports = { childProcess };
