package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	fmt.Print("Part 1: ")
	part1()
	fmt.Print("Part 2: ")
	part2()
}

func part1() {
	content, err := os.ReadFile("input_part_1.txt")
	if err != nil {
		log.Fatal(err)
	}

	lines := strings.Split(string(content), "\n")

	var results []int
	for _, line := range lines {
		firstNumber := getFirstNumber1(line)
		lastNumber := getLastNumber1(line)
		result, err := strconv.Atoi(firstNumber + lastNumber)
		if err != nil {
			panic(err)
		}
		results = append(results, result)
	}

	var sum int
	for _, i := range results {
		sum = sum + i
	}

	fmt.Println(sum)
}

func isInt(str string) bool {
	_, err := strconv.Atoi(str)
	return err == nil
}

func getFirstNumber1(line string) string {
	for i := 0; i < len(line); i++ {
		char := line[i]
		if isInt(string(char)) {
			return string(char)
		}
	}

	return line[len(line)-1:]
}

func getLastNumber1(line string) string {
	for i := len(line) - 1; i >= 0; i-- {
		char := line[i]
		if unicode.IsDigit(rune(char)) {
			return string(char)
		}
	}

	return line[0:]
}

func part2() {
	lookup := map[string]string{
		"zero":  "0",
		"one":   "1",
		"two":   "2",
		"three": "3",
		"four":  "4",
		"five":  "5",
		"six":   "6",
		"seven": "7",
		"eight": "8",
		"nine":  "9",
	}
	content, err := os.ReadFile("input_part_2.txt")
	if err != nil {
		log.Fatal(err)
	}

	lines := strings.Split(string(content), "\n")

	var results []int
	for _, line := range lines {
		firstNumber := getFirstNumber2(line, lookup)
		lastNumber := getLastNumber2(line, lookup)
		result, err := strconv.Atoi(firstNumber + lastNumber)
		if err != nil {
			panic(err)
		}
		results = append(results, result)
	}

	var sum int
	for _, i := range results {
		sum = sum + i
	}

	fmt.Println(sum)
}
func getFirstNumber2(line string, lookup map[string]string) string {
	for i := 0; i < len(line); i++ {
		for j := i + 1; j < len(line); j++ {
			chars := line[i:j]
			if isInt(chars) {
				return string(chars)
			}
			val, ok := lookup[chars]
			if ok {
				return val
			}
		}
	}

	return line[len(line)-1:]
}

func getLastNumber2(line string, lookup map[string]string) string {
	for i := len(line); i >= 0; i-- {
		for j := i - 1; j >= 0; j-- {
			chars := line[j:i]
			if isInt(chars) {
				return string(chars)
			}
			val, ok := lookup[chars]
			if ok {
				return val
			}
		}
	}

	return line[0:]
}
