open Input;

// ABCDE
// DE --> OPCODE
//  C --> Param 1 Mode
//  B --> Param 2 Mode
//  A --> Param 3 Mode
//
// Modes:
// 0 --> Position Mode --> Take value at position dictated by param value [1, 3, 2, 5] if we take element 1 which is 4 we take the 3th element (5)
// 1 --> Immediate Mode --> Take the param as value in above example 3 would be the value instead of 5

let rec handleInput = (numbers, currentIndex) => {
  let result =
    switch(numbers[currentIndex]) {
    | 1 => // Add
    | 2 => // Multiply
    | 3 => // Something
    | 4 => // Else
    | 99 => // Stop
    | _ => raise(Not_found);
  }

  // Perform operation
}

// Part 1
let newInput = Array.copy(myInput);
handleInput(newInput, 0);

