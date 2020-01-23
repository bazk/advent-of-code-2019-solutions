package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"strings"
)

type pwdValue []int

func (val pwdValue) ToInt() int {
	total := 0
	for i := range val {
		total += val[i] * int(math.Pow10(len(val)-i-1))
	}
	return total
}

func (val pwdValue) LessThan(other pwdValue) bool {
	return val.ToInt() < other.ToInt()
}

func (val pwdValue) Increment(pos int) {
	if val[pos] >= 9 {
		val.Increment(pos - 1)
		val[pos] = val[pos-1]
	} else {
		val[pos]++
	}
}

func (val pwdValue) HasDoubleDigits() bool {
	for i := 1; i < len(val); i++ {
		if val[i] == val[i-1] {
			digitBeforeIsDifferent := i <= 1 || val[i-2] != val[i]
			digitAfterIsDifferent := i >= len(val)-1 || val[i+1] != val[i]
			if digitBeforeIsDifferent && digitAfterIsDifferent {
				return true
			}
		}
	}

	return false
}

func (val pwdValue) IsIncreasing() bool {
	for i := 1; i < len(val); i++ {
		if val[i] < val[i-1] {
			return false
		}
	}

	return true
}

type pwdRange struct {
	start pwdValue
	end   pwdValue
}

func parsePwdValue(input string) pwdValue {
	ret := make(pwdValue, len(input))

	for i, digit := range input {
		ret[i] = int(digit) - 48
	}

	return ret
}

func readInput(fileName string) (pwdRange, error) {
	content, err := ioutil.ReadFile(fileName)
	if err != nil {
		return pwdRange{}, fmt.Errorf("failed to read file: %v", fileName)
	}

	items := strings.Split(string(content), "-")
	value0 := parsePwdValue(items[0])
	value1 := parsePwdValue(items[1])
	return pwdRange{value0, value1}, nil
}

func searchSpaceSize(input pwdRange) int {
	count := 0

	n := input.start
	for n.LessThan(input.end) {
		if n.HasDoubleDigits() && n.IsIncreasing() {
			count++
		}
		n.Increment(5)
	}

	return count
}

func main() {
	input, err := readInput(os.Args[1])
	if err != nil {
		return
	}

	fmt.Println("search space size = ", searchSpaceSize(input))
}
