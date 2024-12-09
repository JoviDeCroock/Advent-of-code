async function main(): Promise<void> {
  const test = await Deno.readTextFile("day-9/test.txt");
  const testResult = crunch(test);
  if (testResult !== 1928) {
    throw new Error('Test failed ' + testResult);
  }

  const input = await Deno.readTextFile("day-9/input.txt");
  const result = crunch(input);
  console.log('P1', result);
}

const crunch = (input: string): number => {
  const digits = input.split('').map(Number);
  let id = 0;
  const blocks = [];
  for (let i = 0; i < digits.length; i++) {
    const isOdd = i % 2 === 0;
    if (isOdd) {
      blocks.push(...new Array(digits[i]).fill(id++));
    } else {
      blocks.push(...new Array(digits[i]).fill('.'));
    }
  }

  const blocksCopy = [...blocks]
  for (let i = blocks.length - 1; i >= 0; i--) {
    const block: string = blocks[i];
    if (block === '.') continue;
    blocks.splice(i, 1, '.');
    const index = blocks.findIndex(a => a === '.');
    if (!index) continue;
    blocks[index] = block;
  }

  const checksum = blocks.reduce((acc, block, index) => block === '.' ? acc : acc + (block * index), 0)

  // Part 2:
  const files = new Map<number, number[]>();
  // First, collect all file positions for each identifier
  for (let i = 0; i < blocksCopy.length; i++) {
    const block = blocksCopy[i];
    if (block === '.') continue;
    if (!files.has(block)) {
      files.set(block, []);
    }
    files.get(block)!.push(i);
  }

  // Sort file IDs in descending order
  const fileIds = Array.from(files.keys()).sort((a, b) => b - a);

  // Process each file
  for (const fileId of fileIds) {
    const positions = files.get(fileId)!;
    const fileSize = positions.length;
    
    // Find the current leftmost position of this file
    const currentStart = Math.min(...positions);
    
    // Find all possible free spaces
    let bestPosition = -1;
    let currentPosition = 0;
    
    while (currentPosition < currentStart) {
      let consecutiveDots = 0;
      let validPosition = true;
      
      // Check if we can fit the file here
      for (let i = 0; i < fileSize; i++) {
        if (blocksCopy[currentPosition + i] === '.') {
          consecutiveDots++;
        } else {
          validPosition = false;
          break;
        }
      }
      
      if (validPosition && consecutiveDots >= fileSize) {
        bestPosition = currentPosition;
        break;
      }
      
      currentPosition++;
    }
    
    // If we found a valid position to the left, move the file
    if (bestPosition !== -1) {
      // Clear old positions
      for (const pos of positions) {
        blocksCopy[pos] = '.';
      }
      
      // Place file in new position
      for (let i = 0; i < fileSize; i++) {
        blocksCopy[bestPosition + i] = fileId;
      }
    }
  }

  const checksum2 = blocksCopy.reduce((acc, block, index) => block === '.' ? acc : acc + (block * index), 0);
  console.log('P2', checksum2);
  return checksum;
}

main()
