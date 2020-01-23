package main

import (
	"os"
	"fmt"
	"bufio"
	"math"
	"strconv"
)

func calculateFuel(mass int64) int64 {
	return int64(math.Floor(float64(mass) / 3) - 2)
}

func stabilize(initialFuel int64) int64 {
	var totalFuel int64 = initialFuel
	var remainingMass int64 = initialFuel

	for {
		requiredFuel := calculateFuel(remainingMass)

		if requiredFuel <= 0 {
			break
		}

		totalFuel += requiredFuel
		remainingMass = requiredFuel
	}

	return totalFuel
}

func payloadFuel() int64 {
	var totalFuel int64 = 0

	for scanner := bufio.NewScanner(os.Stdin); scanner.Scan(); {
		mass, _ := strconv.Atoi(scanner.Text())
		fuel := calculateFuel(int64(mass))
		totalFuel += stabilize(fuel)
	}

	return totalFuel
}


func main() {
	// payload := payloadFuel()
	// stabilized := stabilize(payload)
	// fmt.Printf("fuel for payload = %v, stabilized = %v\n",
	// 	payload, stabilized)

	fmt.Println(payloadFuel())
}