open Input;

// ABCDE
// DE --> OPCODE (param 0)
//  C --> Param 1 Mode
//  B --> Param 2 Mode
//  A --> Param 3 Mode
//
// Modes:
// 0 --> Position Mode --> Take value at position dictated by param value [1, 3, 2, 5] if we take element 1 which is 4 we take the 3th element (5)
// 1 --> Immediate Mode --> Take the param as value in above example 3 would be the value instead of 5

let rec makeOmissionExplicit = (code: int) => {
  if (String.length(string_of_int(code)) < 5) {
    makeOmissionExplicit(int_of_string("0" ++ string_of_int(code)));
  } else {
    code;
  }
}

let handleCode = (code: int, param) => {
  let temp = string_of_int(makeOmissionExplicit(code));
  switch(param) {
  | 0 => temp.[4];
  | 1 => temp.[2];
  | 2 => temp.[1];
  | 3 => temp.[0];
  | _ => raise(Not_found);
  }
}
let input = 1;

let rec handleInput = (numbers: array(int), currentIndex) => {
  let code = makeOmissionExplicit(numbers[currentIndex]);

  let modeParam1 = handleCode(code, 1);
  let modeParam2 = handleCode(code, 2);
  let modeParam3 = handleCode(code, 3);

  let value1 = modeParam1 === '0' ? numbers[numbers[currentIndex + 1]] : numbers[currentIndex + 1];
  let value2 = modeParam2 === '0' ? numbers[numbers[currentIndex + 2]] : numbers[currentIndex + 2];
  let value3 = modeParam3 === '0' ? numbers[currentIndex + 3] : currentIndex + 3;

  let result =
    switch(numbers[currentIndex]) {
    | 1 => value1 + value2;
    | 2 => value1 * value2;
    | 3 => input;
    | 4 => value1;
    | 99 => -1
    | _ => raise(Not_found);
  }

  // Perform operation
  if (result === -1) {
    numbers
  } else if (numbers[currentIndex] === 3 || numbers[currentIndex] === 4) {
    numbers[value1] = result;
    handleInput(numbers, currentIndex + 2);
  } else {
    numbers[value3] = result;
    handleInput(numbers, currentIndex + 4);
  }
}

// Part 1
let newInput = Array.copy(myInput);
let result = handleInput(newInput, 0);
Js.log(result)
