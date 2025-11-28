function helperA(x) {
    if (x > 0) {
        return x * 2;
    }
    return 0;
}

function helperB(y) {
    let result = 0;

    for (let i = 0; i < y; i++) {
        result += i;
    }

    return result;
}

function helperC(z) {
    let sum = 0;

    while (sum < z) {
        sum += 1;
    }

    return sum;
}

function helperD(a, b) {
    if (a > b) {
        return a - b;
    } else {
        return b - a;
    }
}

function helperE(arr) {
    let total = 0;

    for (let item of arr) {
        if (item > 0) {
            total += item;
        }
    }

    return total;
}