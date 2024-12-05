async function main(): Promise<void> {
  const test = await Deno.readTextFile("day-5/test.txt");
  const testResult = crunch(test);
  if (testResult !== 143) {
    throw new Error('Test failed');
  }

  const input = await Deno.readTextFile("day-5/input.txt");
  const result = crunch(input);
  console.log('P1', result);
}

const crunch = (input: string): number => {
  const [sequences, printed] = input.split('\n\n');
  const map: Record<number, number[]> = {};
  const incorrect: Array<number[]> = [];
  sequences.split('\n').forEach(sequence => {
    const [before, after] = sequence.split('|');
    if (map[Number(before)]) {
      map[Number(before)].push(Number(after))
    } else {
      map[Number(before)] = [Number(after)]
    }
  })

  let part1 = 0;
  const printedLines = printed.split('\n');
  for (let i = 0; i < printedLines.length; i++) {
    const line = printedLines[i];
    let hasErrored = false;
    // allEntries will contain a list of numbers
    // we have to check whether every number
    const allEntries = line.split(',').map(Number);
    for (let j = 0; j < allEntries.length; j++) {
      const entry = allEntries[j];
      for (let k = 0; k < allEntries.length; k++) {
        if (j === k) {
          continue;
        }

        const comparingEntry = allEntries[k];
        const hasToBeBefore = map[comparingEntry];
        if (k > j && hasToBeBefore && hasToBeBefore.includes(entry)) {
          hasErrored = true;
          break;
        };
      }
    }

    if (!hasErrored) {
      const number = allEntries[Math.floor((allEntries.length - 1) / 2)];
      part1 += number
    } else {
      incorrect.push(allEntries)
    }
  }

  for (let i = 0; i < incorrect.length; i++) {
    incorrect[i].sort((a, b) => {
      const hasToBeAfterA = map[a];
      const hasToBeAfterB = map[b];
      // In map there is a mapping of number --> number[]
      // any number in number[] should be after the number
      // that we find in the key.
      // Here we want to sort an arbitrary list of 
      // numbers to correspond to this ordering.
      if (hasToBeAfterB && hasToBeAfterB.includes(a)) {
        return -1;
      }else if (hasToBeAfterA && hasToBeAfterA.includes(b)) {
        return 1;
      } else {
        return 0;
      }
    });
  }

  console.log('P2', incorrect.reduce((acc, item) => acc + item[Math.floor((item.length - 1) / 2)], 0))

  return part1
}

main()
