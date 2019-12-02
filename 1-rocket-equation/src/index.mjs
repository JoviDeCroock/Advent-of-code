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

console.log('Part 1 Result: ', calculateTotalFuel(allParts));
console.log('Part 2 Result: ', calculateTotalFuelRec(allParts));
