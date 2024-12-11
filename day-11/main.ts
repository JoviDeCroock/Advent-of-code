async function main(): Promise<void> {
  const test = await Deno.readTextFile("day-11/test.txt");
  const testResult = crunch(test, 25);
  if (testResult !== 55312) throw new Error('nope ' + testResult);

  const input = await Deno.readTextFile("day-11/input.txt");
  console.log('p1', crunch(input, 25));
  console.log('p2', crunch(input, 75));
}

/**
f the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
 */
const crunch = (input: string, iterations: number): number => {
  let stoneMap = new Map<number, number>();
  
  input.split(' ').map(Number).forEach(num => {
    stoneMap.set(num, (stoneMap.get(num) || 0) + 1);
  });
  
  for (let i = 0; i < iterations; i++) {
    const newStoneMap = new Map<number, number>();
    
    stoneMap.forEach((count, stone) => {
      const transformed = transform(stone);
      transformed.forEach(newStone => {
        newStoneMap.set(newStone, (newStoneMap.get(newStone) || 0) + count);
      });
    });
    
    stoneMap = newStoneMap;
  }

  return Array.from(stoneMap.values()).reduce((sum, count) => sum + count, 0);
};

const transform = (num: number): number[] => {
  if (num === 0) {
    return [1];
  } else if (num.toString().length % 2 === 0) {
    const numbers = num.toString().split('');
    const half = numbers.length / 2;
    return [Number(numbers.slice(0, half).join('')), Number(numbers.slice(half).join(''))];
  } else {
    return [num * 2024];
  }
}

main();
