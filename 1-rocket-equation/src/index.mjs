import allParts from './input.mjs';

// Helpers
const calculateFuel = (input) => Math.floor(input / 3) - 2;

// Part 1
const calculateTotalFuel = (parts) => parts.reduce((acc, part) => acc + calculateFuel(part), 0);

// Part 2
const calculateFuelRec = (input) => {
  const fuelNeeded = calculateFuel(input);
  if (fuelNeeded > 0) return fuelNeeded + (calculateFuelRec(fuelNeeded) || 0);
}

const calculateTotalFuelRec = (parts) => parts.reduce((acc, part) => acc + calculateFuelRec(part), 0);

console.log('Part 1 Result: ', calculateTotalFuel(allParts));
console.log('Part 2 Result: ', calculateTotalFuelRec(allParts));
