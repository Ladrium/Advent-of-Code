const fs = require('fs');
const read = fs.readFileSync("input.txt");
const data = read.toString().map(Number);
["Part 1", "Part 2"].forEach((x) => require(`./${x}`)(data));