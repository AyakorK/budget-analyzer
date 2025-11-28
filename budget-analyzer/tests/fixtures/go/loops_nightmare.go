package main

func megaCrazyLoops(n int) int {
    result := 0
    temp := 0
    counter := 0

    for i := 0; i < n; i++ {
        for j := 0; j < 10; j++ {
            if j%2 == 0 {
                result += j

                if j > 5 {
                    result += 10
                }
            } else {
                result -= j
            }
        }

        if i > 5 {
            temp += i
        }
    }

    for k := 0; k < n; k++ {
        temp2 := k
        for temp2 > 0 {
            temp2--
            result += temp2

            if temp2 % 3 == 0 {
                counter++
            }
        }
    }

    for result < 100 {
        result++

        if result % 10 == 0 {
            temp++
        }
    }

    while counter < 50 {
        counter++

        if counter % 2 == 0 {
            result += counter
        }
    }

    return result + temp + counter
}

func anotherLoopMess(x int) int {
    total := 0

    for i := 0; i < x; i++ {
        if i % 2 == 0 {
            total += i
        } else {
            total -= i
        }
    }

    for total < 100 {
        total += 10
    }

    return total
}
