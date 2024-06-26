const util = require("util");
const { spawn } = require("node:child_process");

async function childProcess(stringyBoi) {
  return new Promise((resolve, reject) => {
    // const command = spawn("ls", ["-al", "."]);
    // const command = spawn("date");
    const cliArguments = stringyBoi.split(/(\s+)/).filter(function(str) {
      return /\S/.test(str);
    });
    console.log(`cliArguments: ${cliArguments}`);
    const command = spawn(
      "../number-cruncher/target/release/number-cruncher",
      cliArguments,
    );
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
