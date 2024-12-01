async function main(): Promise<void> {
  const test = await Deno.readTextFile("day-1/test.txt");
  const testResult = calculate(test);

  if (testResult.p1 !== 11) {
    throw new Error("P1 FAILED");
  }

  if (testResult.p2 !== 31) {
    throw new Error("P2 FAILED");
  }

  const input = await Deno.readTextFile("day-1/input.txt");
  const result = calculate(input);
  console.log('Day 1: ',  result);
}

function calculate(input: string): { p1: number; p2: number; } {
  const left: Array<number> = [];
  const right: Array<number> = [];
  input.split("\n").forEach((line) => {
    const [leftItem, rightItem] = line.split("   ");
    left.push(Number(leftItem))
    right.push(Number(rightItem))
  })

  left.sort((a, b) => a - b);
  right.sort((a, b) => a - b);

  const differences: Array<number> = []
  for (let i = 0; i < left.length; i++) {
    const leftItem = left[i];
    const rightItem = right[i];

    differences.push(Math.abs(leftItem - rightItem));
  }

  const similarities: Array<number> = []
  for (let i = 0; i < left.length; i++) {
    const leftItem = left[i];

    const rightItem = right.filter((item) => item === leftItem);
    similarities.push(leftItem * rightItem.length)
  }

  return {p1: differences.reduce((acc, item) => acc + item, 0), p2: similarities.reduce((acc, item) => acc + item, 0)};
}

main()
