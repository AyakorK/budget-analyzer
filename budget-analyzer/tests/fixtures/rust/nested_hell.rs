fn nested_hell(x: i32, y: i32, z: i32) -> i32 {
    let mut result = 0;
    let mut temp = 0;

    if x > 0 {
        if y > 0 {
            if z > 0 {
                for i in 0..x {
                    for j in 0..y {
                        for k in 0..z {
                            if i == j {
                                result += 1;

                                if k > 5 {
                                    result += 2;
                                }
                            } else {
                                result += 2;

                                if k < 3 {
                                    result += 1;
                                }
                            }
                        }
                    }
                }
            } else {
                for i in 0..x {
                    result += i;
                }
            }
        } else {
            while result < 50 {
                result += 1;

                if result % 2 == 0 {
                    temp += 1;
                }
            }
        }
    } else {
        for k in 0..10 {
            result += k;

            if k % 2 == 0 {
                temp += k;
            }
        }
    }

    while temp < 100 {
        temp += 1;

        if temp % 5 == 0 {
            result += 5;
        }
    }

    result + temp
}

fn another_nightmare(a: i32, b: i32) -> i32 {
    let mut sum = 0;

    if a > 0 {
        if b > 0 {
            for i in 0..a {
                sum += i;
            }
        }
    }

    while sum < 100 {
        sum += 1;
    }

    sum
}