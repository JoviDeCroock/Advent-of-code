async function main(): Promise<void> {
  const test = await Deno.readTextFile("day-6/test.txt");
  const testResult = crunch(test);
  if (testResult !== 41) {
    throw new Error('Test failed ' + testResult);
  }

  const input = await Deno.readTextFile("day-6/input.txt");
  const result = crunch(input);
  console.log('P1', result);
  
  const result2 = crunchPart2(input);
  console.log('P2', result2);
}

const OBSTRUCTION = '#';

// If there is something directly in front of you, turn right 90 degrees.
// Otherwise, take a step forward.

const GUARD = ['^', '>', 'v', '<'];

const crunch = (input: string): number => {
  const grid = input.split('\n').map(row => row.split(''));
  const guardColIndex = grid.findIndex(row => row.some(cell => GUARD.includes(cell)));
  const guardRowIndex = grid[guardColIndex].findIndex(cell => GUARD.includes(cell));

  let currentPosition = [guardColIndex, guardRowIndex];
  const visited = new Set([currentPosition.join(',')]);

  let currentDirection = GUARD.indexOf(grid[guardColIndex][guardRowIndex]);
  const hasReachedEdge = (): boolean => {
    const [row, col] = currentPosition;
    switch (currentDirection) {
      case 0:
        return row === 0;
      case 1:
        return col === grid[row].length - 1;
      case 2:
        return row === grid.length - 1;
      case 3:
        return col === 0;
      default: {
        throw new Error('lol')
      }
    }
  }

  const getNextPosition = () => {
    const [row, col] = currentPosition;
    switch (currentDirection) {
      case 0:
        return [row - 1, col];
      case 1:
        return [row, col + 1];
      case 2:
        return [row + 1, col];
      case 3:
        return [row, col - 1];
      default: {
        throw new Error('Invalid direction');
      }
    }
  }

  while (!hasReachedEdge()) {
    const nextPosition = getNextPosition()

    const cell = grid[nextPosition[0]][nextPosition[1]];
    if (cell === OBSTRUCTION) {
      currentDirection = (currentDirection + 1) % 4;
    } else {
      visited.add(nextPosition.join(','));
      currentPosition = nextPosition;
    }
  }

  return visited.size
}

const simulatePath = (
  grid: string[][],
  startPos: number[],
  startDirection: number,
  testObstruction: number[]
): boolean => {
  let currentPosition = [...startPos];
  let currentDirection = startDirection;
  const visited = new Set<string>();

  while (true) {
    // Create a state key that includes both position and direction
    const stateKey = `${currentPosition.join(',')},${currentDirection}`;
    
    // If we've seen this state before, we've found a loop
    if (visited.has(stateKey)) {
      return true;
    }
    visited.add(stateKey);

    const nextPosition = getNextPosition(currentPosition, currentDirection);
    
    // Check if we've hit the edge
    if (
      nextPosition[0] < 0 ||
      nextPosition[0] >= grid.length ||
      nextPosition[1] < 0 ||
      nextPosition[1] >= grid[0].length
    ) {
      return false;
    }

    // Check if we hit an obstruction (including our test obstruction)
    if (
      grid[nextPosition[0]][nextPosition[1]] === OBSTRUCTION ||
      (nextPosition[0] === testObstruction[0] && nextPosition[1] === testObstruction[1])
    ) {
      currentDirection = (currentDirection + 1) % 4;
    } else {
      currentPosition = nextPosition;
    }
  }
};

const crunchPart2 = (input: string): number => {
  const grid = input.split('\n').map(row => row.split(''));
  const guardColIndex = grid.findIndex(row => row.some(cell => GUARD.includes(cell)));
  const guardRowIndex = grid[guardColIndex].findIndex(cell => GUARD.includes(cell));
  const startPos = [guardColIndex, guardRowIndex];
  const startDirection = GUARD.indexOf(grid[guardColIndex][guardRowIndex]);

  let loopCount = 0;

  // Try every position on the grid
  for (let row = 0; row < grid.length; row++) {
    for (let col = 0; col < grid[0].length; col++) {
      // Skip existing obstacles, edges, and start position
      if (
        grid[row][col] === OBSTRUCTION ||
        (row === startPos[0] && col === startPos[1])
      ) {
        continue;
      }

      // Test if placing an obstruction here creates a loop
      if (simulatePath(grid, startPos, startDirection, [row, col])) {
        loopCount++;
      }
    }
  }

  return loopCount;
};

// Helper function moved outside to be reusable
const getNextPosition = (currentPosition: number[], currentDirection: number): number[] => {
  const [row, col] = currentPosition;
  switch (currentDirection) {
    case 0:
      return [row - 1, col];
    case 1:
      return [row, col + 1];
    case 2:
      return [row + 1, col];
    case 3:
      return [row, col - 1];
    default: {
      throw new Error('Invalid direction');
    }
  }
};

main()
