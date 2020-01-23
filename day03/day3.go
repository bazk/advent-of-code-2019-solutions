package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type point struct {
	x int
	y int
}

func (p point) String() string {
	return fmt.Sprintf("(%v,%v)", p.x, p.y)
}

type segment struct {
	start point
	end   point
}

func (s segment) String() string {
	return fmt.Sprintf("[%v -> %v]", s.start, s.end)
}

func (s segment) Len() int {
	if s.start.x == s.end.x {
		return abs(s.end.y - s.start.y)
	}

	return abs(s.end.x - s.start.x)
}

type intersection struct {
	p                 point
	manhattanDistance int
	stepsDistance     int
}

func abs(n int) int {
	if n < 0 {
		return -n
	}

	return n
}

func manhattanDistance(p point) int {
	return abs(p.x) + abs(p.y)
}

func createSegment(pos point, cmd string) (segment, error) {
	var ret segment

	dir := cmd[0]
	value, err := strconv.Atoi(cmd[1:])
	if err != nil {
		return ret, fmt.Errorf("failed to parse cmd: %s", cmd)
	}

	ret.start = pos

	switch dir {
	case 'U':
		ret.end = point{pos.x, pos.y + value}
	case 'D':
		ret.end = point{pos.x, pos.y - value}
	case 'L':
		ret.end = point{pos.x - value, pos.y}
	case 'R':
		ret.end = point{pos.x + value, pos.y}
	default:
		return ret, fmt.Errorf("invalid command %s", cmd)
	}

	return ret, nil
}

func readInput(fileName string) [][]segment {
	var paths [][]segment

	file, _ := os.Open(fileName)
	defer file.Close()

	for scanner := bufio.NewScanner(file); scanner.Scan(); {
		line := scanner.Text()
		cmds := strings.Split(line, ",")

		currentPos := point{0, 0}

		segments := make([]segment, len(cmds))

		for index, cmd := range cmds {
			seg, _ := createSegment(currentPos, cmd)
			segments[index] = seg
			currentPos = seg.end
		}

		paths = append(paths, segments)
	}

	return paths
}

func between(v int, start int, end int) bool {
	if end > start {
		return v >= start && v <= end
	}

	return v >= end && v <= start
}

func crosspoint(seg1 segment, seg2 segment) (point, error) {
	if seg1.start.y == seg1.end.y {
		if between(seg2.start.x, seg1.start.x, seg1.end.x) && between(seg1.start.y, seg2.start.y, seg2.end.y) {
			return point{seg2.start.x, seg1.start.y}, nil
		}
		if between(seg2.end.x, seg1.start.x, seg1.end.x) && between(seg1.start.y, seg2.end.y, seg2.end.y) {
			return point{seg2.end.x, seg1.start.y}, nil
		}
	} else {
		if between(seg1.start.x, seg2.start.x, seg2.end.x) && between(seg2.start.y, seg1.start.y, seg1.end.y) {
			return point{seg1.start.x, seg2.start.y}, nil
		}
		if between(seg1.end.x, seg2.start.x, seg2.end.x) && between(seg2.start.y, seg1.end.y, seg1.end.y) {
			return point{seg1.end.x, seg2.start.y}, nil
		}
	}

	return point{0, 0}, fmt.Errorf("no intersection")
}

func checkCollisions(paths [][]segment) []intersection {
	ret := make([]intersection, 0, len(paths[0]))

	path1 := paths[0]
	path2 := paths[1]

	total1 := 0
	for _, seg1 := range path1 {
		total2 := 0
		for _, seg2 := range path2 {
			p, err := crosspoint(seg1, seg2)

			if err == nil && !(p.x == 0 && p.y == 0) {
				stepsDistance := total1 + total2 +
					abs(p.x-seg1.start.x) + abs(p.y-seg1.start.y) +
					abs(p.x-seg2.start.x) + abs(p.y-seg2.start.y)

				i := intersection{p, manhattanDistance(p), stepsDistance}
				ret = append(ret, i)
			}

			total2 += seg2.Len()
		}
		total1 += seg1.Len()
	}

	return ret
}

func main() {
	paths := readInput("input.txt")
	intersections := checkCollisions(paths)

	sort.Slice(intersections, func(i, j int) bool {
		return intersections[i].manhattanDistance < intersections[j].manhattanDistance
	})
	fmt.Println("manhattan = ", intersections[0].manhattanDistance)

	sort.Slice(intersections, func(i, j int) bool {
		return intersections[i].stepsDistance < intersections[j].stepsDistance
	})
	fmt.Println("steps = ", intersections[0].stepsDistance)
}
