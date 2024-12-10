async function main(): Promise<void> {
  const test = await Deno.readTextFile("day-10/test.txt");
  const { score: testScore, rating: testRating } = crunch(test);
  if (testScore !== 36 || testRating !== 81) {
    throw new Error(`Test failed: score ${testScore}, rating ${testRating}`);
  }

  const input = await Deno.readTextFile("day-10/input.txt");
  const { score, rating } = crunch(input);
  console.log('P1', score);
  console.log('P2', rating);
}

const crunch = (input: string): { score: number, rating: number } => {
  const grid = input.split('\n').map(row => row.split('').map(Number));
  const height = grid.length;
  const width = grid[0].length;
  
  // Find all trailheads (height 0)
  const trailheads: [number, number][] = [];
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      if (grid[y][x] === 0) {
        trailheads.push([y, x]);
      }
    }
  }

  // For each trailhead, calculate score and rating
  return trailheads.reduce((acc, [startY, startX]) => {
    const visited = new Set<string>();
    const reachable9s = new Set<string>();
    let trailCount = 0;
    
    const dfs = (y: number, x: number, currentHeight: number) => {
      const key = `${y},${x}`;
      if (y < 0 || y >= height || x < 0 || x >= width || 
          visited.has(key) || grid[y][x] !== currentHeight) {
        return;
      }

      visited.add(key);
      if (currentHeight === 9) {
        reachable9s.add(key);
        trailCount++;
      } else {
        // Try all 4 directions
        [[y-1,x], [y+1,x], [y,x-1], [y,x+1]].forEach(([ny, nx]) => {
          dfs(ny, nx, currentHeight + 1);
        });
      }
      visited.delete(key); // Backtrack to allow other paths
    };

    dfs(startY, startX, 0);
    acc.score += reachable9s.size;
    acc.rating += trailCount;
    return acc;
  }, { score: 0, rating: 0 });
};

main();
