async function main(): Promise<void> {
  const test = await Deno.readTextFile("day-8/test.txt");
  const testResult = crunch(test);
  if (testResult !== 14) {
    throw new Error('Test failed ' + testResult);
  }

  const input = await Deno.readTextFile("day-8/input.txt");
  const result = crunch(input);
  console.log('Result', result);
}

/** Example grid:
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
Expected result: 14
......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.
Where the # is an antinode, we have 14 because the topmost A-frequency antenna overlaps with a 0-frequency antinode.
*/
const crunch = (input: string): number => {
  const lines = input.trim().split('\n');
  const grid = lines.map(line => line.split(''));
  const height = grid.length;
  const width = grid[0].length;
  
  // Groups antennas by their frequency (0, A, etc.)
  const antennasByFreq: Map<string, Array<[number, number]>> = new Map();
  
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const char = grid[y][x];
      if (char !== '.') {
        if (!antennasByFreq.has(char)) {
          antennasByFreq.set(char, []);
        }
        antennasByFreq.get(char)!.push([x, y]);
      }
    }
  }

  const antinodes = findAntinodes1(antennasByFreq, grid);
  const antinodes2 = findAntinodes2(antennasByFreq, grid);

  console.log(antinodes2.size)

  return antinodes.size;
}

const findAntinodes2 = (nodes: Map<string, Array<[number, number]>>, map: string[][]) => {
  const antinodes = new Set<string>();
  [...nodes.values()].forEach(nodeGroup =>
    nodeGroup.forEach(([x1, y1], i) =>
      [...nodeGroup.values()]
        .filter((_, j) => i < j)
        .forEach(([x2, y2]) => {
          const div = gcd(Math.abs(x2 - x1), Math.abs(y2 - y1));
          const [dx, dy] = [(x2 - x1) / div, (y2 - y1) / div];
          let [x, y] = [x1, y1];
          while (map[y]?.[x]) {
            antinodes.add(`${x}-${y}`);
            x -= dx;
            y -= dy;
          }
          [x, y] = [x1 + dx, y1 + dy];
          while (map[y]?.[x]) {
            antinodes.add(`${x}-${y}`);
            x += dx;
            y += dy;
          }
        }),
    ),
  );

  return antinodes;
};

const gcd = (a: number, b: number): number => {
  a = Math.abs(a);
  b = Math.abs(b);
  while (b) {
    const t = b;
    b = a % b;
    a = t;
  }
  return a;
};

const findAntinodes1 = (nodes: Map<string, Array<[number, number]>>, map: string[][]) => {
  const antinodes = new Set<string>();
  
  // Iterate over antenna groups
  for (const nodeGroup of nodes.values()) {
    const groupLength = nodeGroup.length;
    
    // Compare each antenna with others in its group
    for (let i = 0; i < groupLength; i++) {
      const [x1, y1] = nodeGroup[i];
      
      for (let j = 0; j < groupLength; j++) {
        if (i === j) continue;
        
        const [x2, y2] = nodeGroup[j];
        // Calculate antinode position
        const antinodeX = 2 * x1 - x2;
        const antinodeY = 2 * y1 - y2;
        
        // Check if antinode is in bounds and valid
        if (map[antinodeY]?.[antinodeX]) {
          antinodes.add(`${antinodeX}-${antinodeY}`);
        }
      }
    }
  }
  
  return antinodes;
};

main()
