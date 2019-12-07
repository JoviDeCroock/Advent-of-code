open Input;

// ABCDE
// DE --> OPCODE (param 0)
//  C --> Param 1 Mode
//  B --> Param 2 Mode
//  A --> Param 3 Mode
//
// Modes:
// 0 --> Position Mode --> Take value at position dictated by param value [1, 3, 2, 5] if we take element 1 which is 3 we take the 3th element (5)
// 1 --> Immediate Mode --> Take the param as value in above example 3 would be the value instead of 5

let rec makeOmissionExplicit = (code: string) => {
  if (String.length(code) < 5) {
    makeOmissionExplicit("0" ++ code);
  } else {
    code;
  }
}

let handleCode = (code: string, param) => {
  let temp = makeOmissionExplicit(code);
  let res = switch(param) {
  | 0 => temp.[4];
  | 1 => temp.[2];
  | 2 => temp.[1];
  | 3 => temp.[0];
  | _ => raise(Not_found);
  }
  String.make(1, res);
}

let input = 1;

let rec handleInput = (numbers: array(int), currentIndex) => {
  let code = string_of_int(numbers[currentIndex]);

  let modeParam1 = handleCode(code, 1);
  let modeParam2 = handleCode(code, 2);
  let modeParam3 = handleCode(code, 3);
  let opCode = handleCode(code, 0);
  let result =
    switch(opCode) {
    | "1" => (modeParam1 === "0" ? numbers[numbers[currentIndex + 1]] : numbers[currentIndex + 1]) + (modeParam2 === "0" ? numbers[numbers[currentIndex + 2]] : numbers[currentIndex + 2]);
    | "2" => (modeParam1 === "0" ? numbers[numbers[currentIndex + 1]] : numbers[currentIndex + 1]) * (modeParam2 === "0" ? numbers[numbers[currentIndex + 2]] : numbers[currentIndex + 2]);
    | "3" => input;
    | "4" => (modeParam1 === "0" ? numbers[numbers[currentIndex + 1]] : numbers[currentIndex + 1]);
    | "9" => -1
    | _ => raise(Not_found);
  }

  // Perform operation
  if (result === -1) {
    numbers
  } else if (opCode === "3") {
    numbers[numbers[currentIndex + 1]] = result;
    handleInput(numbers, currentIndex + 2);
  } else if (opCode === "4") {
    Js.log("--- OUTPUT ---");
    Js.log(result);
    handleInput(numbers, currentIndex + 2);
  } else {
    let value3 = modeParam3 === "0" ? numbers[currentIndex + 3] : currentIndex + 3;
    numbers[value3] = result;
    handleInput(numbers, currentIndex + 4);
  }
}

// Part 1
let newInput = Array.copy(myInput);
let result = handleInput(newInput, 0);
