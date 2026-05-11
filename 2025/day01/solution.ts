import * as fs from "fs";
import * as path from "path";

const input = fs.readFileSync(path.join(__dirname, "input.txt"), "utf-8").trim();
const lines = input.split("\n");

const mod100 = (n: number) => ((n % 100) + 100) % 100;

// Part 1
function part1(): number {
  let progress = 50;
  return lines.reduce((acc, value) => {
    const [direction, ...numbers] = value;
    const amount = Number(numbers.join('')) % 100
    progress = mod100(progress + (direction === "R" ? amount : -amount));  
    return progress === 0 ? acc + 1 : acc
  }, 0)
}

// Part 2
function part2(): number {
  let progress = 50;
  return lines.reduce((acc, value) => {
    const [direction, ...numbers] = value;
    const amount = Number(numbers.join(''));
    if (direction === 'L') {
      const zeroes = Math.floor((amount + ((100 - progress) % 100)) / 100);                                                                                                                                
      progress = mod100(progress - amount);
      return acc + zeroes;
    } else {
      const zeroes = Math.floor((progress + amount) / 100);                                                                                                                                                
      progress = mod100(progress + amount);
      return acc + zeroes;
    }
  }, 0)
}

console.log("Part 1:", part1());
console.log("Part 2:", part2());
