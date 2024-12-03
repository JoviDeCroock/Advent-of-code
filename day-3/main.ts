async function main(): Promise<void> {
  const test = await Deno.readTextFile("day-3/test.txt");
  const testResult = calculate(test);
  if (testResult !== 161) {
    throw new Error('Test failed');
  }

  const test2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
  const test2Result = calculateWithDoAndDont(test2);
  if (test2Result !== 48) {
    throw new Error('Test failed');
  }

  const input = await Deno.readTextFile("day-3/input.txt");
  let result = calculate(input);
  console.log('Part 1', result);
  result = calculateWithDoAndDont(input);
  console.log('Part 2', result);
}

const calculate = (input: string): number => {
  const actions = input.matchAll(/(mul\(\d+,\d+\))/g);
  let result = 0;
  for (const action of actions) {
    const [captured] = action;

    const [x, y] = captured.replace("mul(", "").replace(")", "").split(",");
    result += Number(x) * Number(y);
  }

  return result;
}

const calculateWithDoAndDont = (input: string): number => {
  const actions = input.matchAll(/((mul\(\d+,\d+\)|don\'t\(\)|do\(\)))/g);
  let result = 0;
  let capturing = true;
  for (const action of actions) {
    const [captured] = action;
    if (captured === "do()") {
      capturing = true;
      continue;
    } else if (captured === "don't()") {
      capturing = false;
      continue;
    } else if (!capturing) {
      continue;
    }

    const [x, y] = captured.replace("mul(", "").replace(")", "").split(",");
    result += Number(x) * Number(y);
  }

  return result;
}

main()
