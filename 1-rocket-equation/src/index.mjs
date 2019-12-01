import allParts from './input.mjs';

const DIVISION = 3;
const SUBTRACTION = 2;

// Helpers
const roundDown = (input) => Math.floor(input);
const divideBy = (input, x) => input / x;
const subtract = (input, x) => input - x;
const calculateFuel = (input) => subtract(roundDown(divideBy(input, DIVISION)), SUBTRACTION);

// Part 1
const calculateTotalFuel = (parts) => parts.reduce((acc, part) => acc + calculateFuel(part), 0);

// Part 2
const calculateFuelRec = (input, total = 0) => {
  const fuelNeeded = calculateFuel(input);
  if (fuelNeeded > 0) return calculateFuelRec(fuelNeeded , total + fuelNeeded)
  return total;
}

const calculateTotalFuelRec = (parts) => parts.reduce((acc, part) => acc + calculateFuelRec(part), 0);

console.log('--- PART 1 ---');
console.log('test 1:', calculateTotalFuel([12]), ' should be 2');
console.log('test 2:', calculateTotalFuel([14]), ' should be 2');
console.log('test 3:', calculateTotalFuel([1969]), ' should be 654');
console.log('test 4:', calculateTotalFuel([100756]), ' should be 33583');
console.log('Final test ', calculateTotalFuel(allParts));
console.log('--- PART 2 ---');
console.log('test 1:', calculateTotalFuelRec([12]), ' should be 2');
console.log('test 2:', calculateTotalFuelRec([14]), ' should be 2');
console.log('test 3:', calculateTotalFuelRec([1969]), ' should be 966');
console.log('test 4:', calculateTotalFuelRec([100756]), ' should be 50346');
console.log('Final test ', calculateTotalFuelRec(allParts));
