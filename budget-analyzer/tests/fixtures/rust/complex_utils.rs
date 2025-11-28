fn complex_calculation(a: i32, b: i32, c: i32) -> i32 {
    let mut result = 0;

    if a > 0 {
        if b > 0 {
            if c > 0 {
                result = a + b + c;
            } else {
                result = a + b;
            }
        } else {
            result = a;
        }
    }

    for i in 0..10 {
        result += i;
    }

    for i in 0..10 {
        result += i;
    }

    let mut counter = result;
    while counter < 100 {
        counter += 1;
    }

    counter
}

fn another_complex_function(x: i32) -> i32 {
    let mut temp = 0;

    for j in 0..x {
        if j % 2 == 0 {
            temp += j;
        } else {
            temp -= j;
        }
    }

    temp
}

fn yet_another_function(arr: Vec<i32>) -> i32 {
    let mut sum = 0;

    for item in arr {
        if item > 0 {
            sum += item;
        }
    }

    sum
}

fn validator(value: i32) -> bool {
    if value < 0 {
        return false;
    }

    if value > 1000 {
        return false;
    }

    if value % 2 != 0 {
        return false;
    }

    true
}

fn processor(data: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    for item in data {
        if validator(item) {
            result.push(item * 2);
        } else {
            result.push(item);
        }
    }

    result
}