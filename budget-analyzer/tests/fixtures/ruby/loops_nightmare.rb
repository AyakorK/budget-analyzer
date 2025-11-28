def crazy_loops(n)
    result = 0

    while result < n
        for i in 0..10
            if i % 2 == 0
                result += i
            end
        end

        result += 1
    end

    for j in 0..n
        while j > 0
            j -= 1
            result += j
        end
    end

    return result
end