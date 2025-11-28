function megaComplexCalculation(a: number, b: number, c: number, d: number, e: number): number {
  let result = 0;
  let temp = 0;
  let counter = 0;

  if (a > 0) {
    if (b > 0) {
      if (c > 0) {
        if (d > 0) {
          if (e > 0) {
            result = a + b + c + d + e;
          } else {
            result = a + b + c + d;
          }
        } else {
          result = a + b + c;
        }
      } else {
        result = a + b;
      }
    } else {
      result = a;
    }
  }

  for (let i = 0; i < 10; i++) {
    result += i;

    if (result > 50) {
      result -= 5;
    }
  }

  while (counter < 100) {
    counter += 1;

    if (counter % 2 === 0) {
      temp += counter;
    }
  }

  for (let j = 0; j < 20; j++) {
    temp += j;
  }

  return result + temp + counter;
}

function anotherComplexFunction(x: number, y: number): number {
  let temp = 0;
  let multiplier = 1;

  for (let j = 0; j < x; j++) {
    if (j % 2 === 0) {
      temp += j;
    } else {
      temp -= j;
    }

    if (j > 10) {
      multiplier = 2;
    }
  }

  while (temp < y) {
    temp += multiplier;
  }

  return temp;
}

function yetAnotherFunction(arr: number[]): number {
  let sum = 0;
  let count = 0;

  for (let item of arr) {
    if (item > 0) {
      sum += item;
      count++;
    } else {
      sum -= item;
    }
  }

  if (count > 10) {
    sum = sum * 2;
  }

  return sum;
}

function validator(value: number): boolean {
  if (value < 0) {
    return false;
  }

  if (value > 1000) {
    return false;
  }

  if (value % 2 !== 0) {
    return false;
  }

  if (value < 10) {
    return false;
  }

  return true;
}

function processor(data: number[]): number[] {
  let result: number[] = [];

  for (let item of data) {
    if (validator(item)) {
      result.push(item * 2);
    } else {
      if (item < 0) {
        result.push(0);
      } else {
        result.push(item);
      }
    }
  }

  return result;
}

function ultimateProcessor(data: number[]): number {
  let total = 0;
  let processed = processor(data);

  for (let value of processed) {
    if (value > 100) {
      total += value;
    }
  }

  while (total > 1000) {
    total = total / 2;
  }

  return total;
}
