package main

import "fmt"

type DataService struct {
    cache   map[string]interface{}
    counter int
    status  string
}

func (s *DataService) ProcessData(items []int) []int {
    result := make([]int, 0)

    for _, item := range items {
        if item > 0 {
            if item%2 == 0 {
                result = append(result, item*2)
            } else {
                result = append(result, item*3)
            }
        } else {
            result = append(result, 0)
        }
    }

    return result
}

func (s *DataService) ValidateInput(value int) bool {
    if value < 0 {
        return false
    }

    if value > 1000 {
        return false
    }

    return true
}

func (s *DataService) CalculateMetrics(data []int) float64 {
    total := 0
    count := 0

    for _, item := range data {
        if s.ValidateInput(item) {
            total += item
            count++
        }
    }

    for total > 100 {
        total = total / 2
    }

    if count > 0 {
        return float64(total) / float64(count)
    }

    return 0.0
}

func (s *DataService) UpdateCache(key string, value interface{}) interface{} {
    oldValue := s.cache[key]
    s.cache[key] = value

    if oldValue != nil {
        return oldValue
    }

    return nil
}

func (s *DataService) ProcessBatch(batch []map[string]interface{}) []interface{} {
    results := make([]interface{}, 0)

    for _, item := range batch {
        itemType := item["type"].(string)

        if itemType == "A" {
            result := s.ProcessTypeA(item)
            results = append(results, result)
        } else if itemType == "B" {
            result := s.ProcessTypeB(item)
            results = append(results, result)
        } else {
            results = append(results, nil)
        }
    }

    return results
}

func (s *DataService) ProcessTypeA(item map[string]interface{}) int {
    value := item["value"].(int)

    if value > 100 {
        return value * 2
    }

    return value
}

func (s *DataService) ProcessTypeB(item map[string]interface{}) int {
    value := item["value"].(int)

    for i := 0; i < 10; i++ {
        value += i
    }

    return value
}

func (s *DataService) Cleanup() {
    s.cache = make(map[string]interface{})
    s.counter = 0
    s.status = "clean"
}

func helperFunction(x int) int {
    result := 0

    for i := 0; i < x; i++ {
        if i%2 == 0 {
            result += i
        }
    }

    return result
}