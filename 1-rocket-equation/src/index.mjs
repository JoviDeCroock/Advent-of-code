import allParts from './input.mjs';

const DIVISION = 3;
const SUBTRACTION = 2;

const roundDown = (input) => Math.floor(input);
const divideBy = (input, x) => input / x;
const subtract = (input, x) => input - x;

const calculateFuelRequirement = (parts) => parts.reduce((acc, part) => acc + subtract(roundDown(divideBy(part, DIVISION)), SUBTRACTION), 0);

console.log('test 1:', calculateFuelRequirement([12]), ' should be 2');
console.log('test 2:', calculateFuelRequirement([14]), ' should be 2');
console.log('test 3:', calculateFuelRequirement([1969]), ' should be 654');
console.log('test 4:', calculateFuelRequirement([100756]), ' should be 33583');
console.log('Final test ', calculateFuelRequirement(allParts));
