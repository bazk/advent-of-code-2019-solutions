package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

type Memory []int

func createMemory(size int) Memory {
	return make(Memory, size)
}

func debugMemory(memory Memory) {
	fmt.Println(memory[:32], " ...")
}

func flashProgram(memory Memory, programFile string) error {
	content, err := ioutil.ReadFile(programFile)
	if err != nil {
		return fmt.Errorf("failed to read program file: %v", programFile)
	}

	items := strings.Split(string(content), ",")

	for index, item := range items {
		value, err := strconv.Atoi(item)
		if err != nil {
			return fmt.Errorf("failed to parse program byte at index %v", index)
		}

		memory[index] = value
	}

	return nil
}

func runProgram(memory Memory) error {
	var pc int = 0

	for {
		switch memory[pc] {
		case 1:
			result := memory[pc+3]
			memory[result] = memory[memory[pc+1]] + memory[memory[pc+2]]
			pc += 4

		case 2:
			result := memory[pc+3]
			memory[result] = memory[memory[pc+1]] * memory[memory[pc+2]]
			pc += 4

		case 99:
			return nil

		default:
			return fmt.Errorf("unknown opcode: %v", memory[pc])
		}
	}
}

func part1() int {
	memory := createMemory(256)

	flashProgram(memory, "input.txt")

	memory[1] = 12
	memory[2] = 02

	runProgram(memory)

	return memory[0]
}

func part2() (int, error) {
	target := 19690720

	for noun := 0; noun < 99; noun++ {
		for verb := 0; verb < 99; verb++ {
			memory := createMemory(256)
			flashProgram(memory, "input.txt")
			memory[1] = noun
			memory[2] = verb
			runProgram(memory)

			if memory[0] == target {
				return noun*100 + verb, nil
			}
		}
	}

	return 0, fmt.Errorf("answer not found")
}

func main() {
	fmt.Printf("Answer for part 1: %v\n", part1())

	p2, err := part2()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Answer for part 2: %v\n", p2)
}
