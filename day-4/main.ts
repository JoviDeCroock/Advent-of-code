async function main(): Promise<void> {
  const test = await Deno.readTextFile("day-4/test.txt");
  const testGrid = makeGrid(test, '|');
  const testResult = part1(testGrid);
  if (testResult !== 18) {
    throw new Error('Test failed');
  }

  const input = await Deno.readTextFile("day-4/input.txt");
  const grid = makeGrid(input, '|');
  const result = part1(grid);
  console.log('RESULT', result);
  console.log('RESULT 2', part2(grid));

}

function makeGrid(inp: string, border: string) {
  const lines = inp.split("\n");

  const res = [];
  res.push(border.repeat(lines[0].length + 2).split(""));
  for (const line of lines) {
    res.push((border + line + border).split(""));
  }
  res.push(border.repeat(lines[0].length + 2).split(""));

  return res;
}


function part2(grid: string[][]) {
  let res = 0;
  for (let i = 1; i < grid.length - 1; i++) {
    for (let j = 1; j < grid[i].length - 1; j++) {
      if (grid[i][j] === "A") {
        const a = grid[i - 1][j - 1] + grid[i + 1][j + 1];

        const b = grid[i - 1][j + 1] + grid[i + 1][j - 1];

        if ((a === "MS" || a === "SM") && (b === "MS" || b === "SM")) {
          res++;
        }
      }
    }
  }

  return res;
}

function part1(grid: string[][]) {
  const directions = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
  ];

  let res = 0;
  for (let i = 1; i < grid.length - 1; i++) {
    for (let j = 1; j < grid[i].length - 1; j++) {
      directions.forEach((dir) => {
        let ii = i;
        let jj = j;
        let word = grid[i][j];

        while ("XMAS".startsWith(word)) {
          ii = ii + dir[0];
          jj = jj + dir[1];

          word += grid[ii][jj];

          if (word === "XMAS") {
            res++;
            break;
          }
        }
      });
    }
  }

  return res;
}

main()
