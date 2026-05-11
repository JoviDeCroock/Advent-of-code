#!/usr/bin/env ts-node
/**
 * Usage: npm run new -- <year> <day>
 * Example: npm run new -- 2024 1
 *
 * Creates:
 *   <year>/day<dd>/solution.ts
 *   <year>/day<dd>/input.txt
 */
import * as fs from "fs";
import * as path from "path";

const args = process.argv.slice(2).filter((a) => a !== "--");
const [yearArg, dayArg] = args;

if (!yearArg || !dayArg) {
  console.error("Usage: npm run new -- <year> <day>");
  process.exit(1);
}

const year = yearArg;
const day = dayArg.padStart(2, "0");
const dir = path.join(__dirname, "..", year, `day${day}`);

if (fs.existsSync(dir)) {
  console.error(`Directory already exists: ${dir}`);
  process.exit(1);
}

fs.mkdirSync(dir, { recursive: true });

fs.writeFileSync(path.join(dir, "input.txt"), "");

fs.writeFileSync(
  path.join(dir, "solution.ts"),
  `import * as fs from "fs";
import * as path from "path";

const input = fs.readFileSync(path.join(__dirname, "input.txt"), "utf-8").trim();
const lines = input.split("\\n");

// Part 1
function part1(): number {
  return 0;
}

// Part 2
function part2(): number {
  return 0;
}

console.log("Part 1:", part1());
console.log("Part 2:", part2());
`
);

console.log(`Created ${year}/day${day}/`);
