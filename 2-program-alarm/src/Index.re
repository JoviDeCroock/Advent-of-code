open Input;

// Common logic
let add = (first, second) => first + second;
let multiply = (first, second) => first * second;

let rec handleInput = (numbers, currentIndex) => {
  let result =
    switch(numbers[currentIndex]) {
    | 1 => add(numbers[numbers[currentIndex + 1]], numbers[numbers[currentIndex + 2]])
    | 2 => multiply(numbers[numbers[currentIndex + 1]], numbers[numbers[currentIndex + 2]]);
    | 99 => -1;
    | _ => raise(Not_found);
  }

  if (result == -1) {
    numbers;
  } else {
    numbers[numbers[currentIndex + 3]] = result;
    handleInput(numbers, currentIndex + 4);
  }
}

// Part 1
let newInput = Array.copy(myInput);
newInput[1] = 12;
newInput[2] = 2;
Js.log(handleInput(newInput, 0)[0]);

// Part 2
let prepare = (list, noun, verb) => {
  let newList = Array.copy(list);
  newList[1] = noun;
  newList[2] = verb;
  handleInput(newList, 0)
};

let ex2 = (inputList, tar) => {
  for (noun in 0 to 99) {
    for (verb in 0 to 99) {
      let res = try(prepare(inputList, noun, verb)) {
        | Not_found => [|0|]
      };

      if (res[0] === tar) {
        Js.log("answer");
        Js.log((100 * noun) + verb);
      }
    }
  }
}

ex2(myInput, 19690720);

