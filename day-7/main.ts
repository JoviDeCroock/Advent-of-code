async function main(): Promise<void> {
  const test = await Deno.readTextFile("day-7/test.txt");
  const testResult = crunch(test);
  if (testResult.p1 !== 3749) {
    throw new Error('Test failed ' + testResult.p1);
  }

  if (testResult.p2 !== 11387) {
    throw new Error('Test failed ' + testResult.p2);
  }

  const input = await Deno.readTextFile("day-7/input.txt");
  const result = crunch(input);
  console.log('Result', result);
}

// If there is something directly in front of you, turn right 90 degrees.
// Otherwise, take a step forward.
const crunch = (input: string): { p1: number; p2: number; } => {
  const sequence = input.split('\n');
  let p2 = 0;
  let p1 = 0;
  for (let i = 0; i < sequence.length; i++) {
    const line = sequence[i];
    const [stringTarget, stringifiedNumbers] = line.split(':')
    const target = Number(stringTarget);
    const numbers = stringifiedNumbers.split(' ').map(Number);
    // We have to check whether all numbers evaluated from left to right
    // are equal to the target, we can only multiply and add.
    // Try all possible combinations of operations
    if (findValidOperations(numbers, target, false)) {
      p1 += target;
    }

    if (findValidOperations(numbers, target, true)) {
      p2 += target;
    }
  }

  return { p1, p2 };
}

function findValidOperations(numbers: number[], target: number, withConcat: boolean): boolean {
  // For n numbers, we need n-1 operators
  const operatorCount = numbers.length - 1;
  
  // Try all possible combinations of operators
  // Each position can be either MUL (0), ADD (1), or CONCAT (2)
  const maxCombinations = Math.pow(withConcat ? 3 : 2, operatorCount);
  
  for (let i = 0; i < maxCombinations; i++) {
    // Convert number to base-3 to get operator combination
    const operators: string[] = [];
    let temp = i;
    for (let j = 0; j < operatorCount; j++) {
      if (withConcat) {
        operators.push(temp % 3 === 0 ? 'MUL' : temp % 3 === 1 ? 'ADD' : 'CONCAT');
      } else {
        operators.push(temp % 2 === 0 ? 'MUL' : 'ADD');
      }
      temp = Math.floor(temp / (withConcat ? 3 : 2));
    }
    
    // Evaluate expression left-to-right
    let result = numbers[0];
    for (let j = 0; j < operators.length; j++) {
      if (operators[j] === 'MUL') {
        result *= numbers[j + 1];
      } else if (operators[j] === 'ADD') {
        result += numbers[j + 1];
      } else { // CONCAT
        result = Number(String(result) + String(numbers[j + 1]));
      }
    }
    
    if (result === target) {
      return true;
    }
  }
  
  return false;
}

main()
