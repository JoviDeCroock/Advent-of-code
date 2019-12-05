import bounds from './input.mjs';

function hasOnlyGrowth(candidate) {
  const len = candidate.length;
  for (let i = 0; i < len - 1; i++) {
    if (Number(candidate[i]) > Number(candidate[i + 1])) return false;
  }
  return true;
}

function hasDupe(candidate) {
  const len = candidate.length;

  for (let i = 0; i < len - 1; i++) {
    if (candidate[i] === candidate[i + 1]) {
      return true;
    }
  }
  return false;
}

// Part 1
const checkPossibilities = (input) => {
  const [lowerBound, upperBound] = input.split('-');

  let possibilities = 0;
  for (let i = lowerBound; i <= upperBound; i++) {
    if (hasDupe(`${i}`) && hasOnlyGrowth(`${i}`)) possibilities++;
  }

  return possibilities;
}

// Part 2
function hasValidDoubleDigits(candidate) {
  for (let i = 0, len = candidate.length; i < len - 1; i++) {
    if (
      candidate[i] === candidate[i + 1] &&
      !hasBefore(candidate, i) &&
      !hasAfter2(candidate, i)
    ) {
      return true;
    }
  }

  return false;
}

function hasBefore(candidate, i) {
  if (i === 0) return false;
  return candidate[i] === candidate[i - 1];
}

function hasAfter2(candidate, i) {
  if (i === candidate.length - 2) return false;
  return candidate[i] === candidate[i + 2];
}

const checkPossibilities2 = (input) => {
  const [lowerBound, upperBound] = input.split('-');

  let possibilities = 0;
  for (let i = lowerBound; i <= upperBound; i++) {
    if (hasValidDoubleDigits(`${i}`) && hasOnlyGrowth(`${i}`)) possibilities++;
  }

  return possibilities;
}

console.log('Part 1 Result: ', checkPossibilities(bounds));
console.log('Part 2 Result: ', checkPossibilities2(bounds));
