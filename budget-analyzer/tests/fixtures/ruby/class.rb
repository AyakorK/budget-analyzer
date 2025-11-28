class Point
    attr_accessor :x, :y

    def initialize(x, y)
        @x = x
        @y = y
    end

    def distance
        Math.sqrt(@x * @x + @y * @y)
    end
end
