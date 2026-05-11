import * as fs from "fs";
import * as path from "path";

const input = fs.readFileSync(path.join(__dirname, "input.txt"), "utf-8").trim();

const inclusiveRange = (first: number, last: number): Array<number> => {
  const difference = (last - first) + 1;
  return new Array(difference).fill(0).map((_, i) => first + i);
} 

// Part 1
function part1(): number {
  const values = input.split(',');
  return values.reduce((acc, value) => {
    const [first, last] = value.split('-');
    const numbers = inclusiveRange(Number(first), Number(last));
    return acc + numbers.reduce((acc, item) => {
      const stringified = String(item);
      if (stringified.length % 2 === 0) {
        const divider = stringified.length / 2;
        const first = Number(stringified.slice(0, divider))
        const last = Number(stringified.slice(divider))
        return first === last ? acc + item : acc;
      }

      return acc;
    }, 0)
  }, 0);
}

const findAllDividers = (item: number): Array<number> => {
  const stringified = String(item);
  let start = Math.floor(stringified.length / 2);
  const dividers: number[] = [];
  while (start - 1 !== -1) {
    if (stringified.length % start == 0) {
      dividers.push(start);
    }
    start -= 1;
  }

  return dividers;
}

function getAllNumbers(stringified: string, divider: number): number[] {
  const amount = stringified.length / divider;
  return new Array(amount).fill(0).map((value, index) => Number(stringified.slice(index * divider, (index + 1) * divider)))
}

// Part 2
function part2(): number {
  const values = input.split(',');
  return values.reduce((acc, value) => {
    const [first, last] = value.split('-');
    const numbers = inclusiveRange(Number(first), Number(last));
    return acc + numbers.reduce((acc, item) => {
      const dividers = findAllDividers(item);
      const stringified = String(item);
      
      let isInvalid = false;
      for (const divider of dividers) {
        const numbers = getAllNumbers(stringified, divider)
        isInvalid = numbers.every((value, index) => {
          if (index === 0) return true;
          return value === numbers[index - 1]
        })
        if (isInvalid) break;
      }
      return isInvalid ? acc + item : acc;
    }, 0)
  }, 0);
}

console.log("Part 1:", part1());
console.log("Part 2:", part2());
