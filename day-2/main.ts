async function main(): Promise<void> {
  const test = await Deno.readTextFile("day-2/test.txt");
  const testReports = crunchData(test);
  const testReportsWithErrors = crunchDataWithRemoval(test);
  if (testReports.length !== 2) {
    throw new Error("Invalid test data");
  }

  if (testReportsWithErrors.length !== 4) {
    throw new Error("Invalid test data 2");
  }

  const input = await Deno.readTextFile("day-2/input.txt");
  const reports = crunchData(input);
  const reportsWithErrors = crunchDataWithRemoval(input);
  console.log('Day 2: ', reports.length, reportsWithErrors.length);

}

const crunchData = (input: string): Array<Array<number>> => {
  const reports = input.split("\n");
  const validReports: Array<Array<number>> = [];
  for (let i = 0; i < reports.length; i++) {
    const numbers: Array<number> = reports[i].split(" ").map(Number);
    if (isValidSequence(numbers)) {
      validReports.push(numbers);
    }

  }
  return validReports
}

const crunchDataWithRemoval = (input: string): Array<Array<number>> => {
  const reports = input.split("\n");
  const validReports: Array<Array<number>> = [];
  for (let i = 0; i < reports.length; i++) {
    const numbers: Array<number> = reports[i].split(" ").map(Number);
    let isValid = isValidSequence(numbers);
    if (isValid) {
      validReports.push(numbers);
      continue;
    }

    for (let ignored = 0; ignored < numbers.length; ignored++) {
      const newNumbers = [...numbers];
      newNumbers.splice(ignored, 1);
      const isValidSubSequence = isValidSequence(newNumbers);
      if (isValidSubSequence) {
        isValid = true;
      }
    }

    if (isValid) {
      validReports.push(numbers);
    }
  }

  return validReports
}

const isValidSequence = (numbers: Array<number>): boolean => {
  let isValid = true;
  let mode: 'increasing' | 'decreasing' = 'decreasing';
  for (let j = 0; j < numbers.length; j++) {
    if (!isValid) continue;

    if (j === 0) {
      // Do nothing
    } else {
      const previous = numbers[j - 1];
      const current = numbers[j];
      const diff = Math.abs((previous as number) - current);
      if (j === 1) {
        mode = previous < current ? 'increasing' : 'decreasing';
      }

      if (previous === current) {
        isValid = false
      } else if (mode === 'increasing' && previous > current) {
        isValid = false
      } else if (mode === 'decreasing' && previous < current) {
        isValid = false
      } else if (diff > 3 || diff < 1) {
        isValid = false
      } else if (j === numbers.length - 1) {
        isValid = true;
      }
    }
  }

  return isValid;
}


main()
