package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)
func check(e error) {
    if e != nil {
        panic(e)
    }
}
func main() {
	data,err := os.ReadFile("./input.txt")

	check(err)
	part1(string(data))
	part2(string(data))
}

func part1(input string) {
	lines:= strings.Split(input, "\n")
	sum := 0
	for i:=0;i<len(lines);i++ {
		numbers:= strings.Split(lines[i], "x")
		a, err:= strconv.Atoi(numbers[0])
		check(err)
		b,err:= strconv.Atoi(numbers[1])
		check(err)
		c,err:= strconv.Atoi(numbers[2])
		check(err)
		fmt.Println(a,b,c,2*a*b + 2*a*c + 2*c*b + min(a,b,c))
		sum += 2*a*b + 2*a*c + 2*c*b + min(a*b,a*c,c*b)
	}
	fmt.Print(sum)
}

func part2(input string) {
	lines:= strings.Split(input, "\n")
	sum := 0
	for i:=0;i<len(lines);i++ {
		numbers:= strings.Split(lines[i], "x")
		a, err:= strconv.Atoi(numbers[0])
		check(err)
		b,err:= strconv.Atoi(numbers[1])
		check(err)
		c,err:= strconv.Atoi(numbers[2])
		check(err)
		fmt.Println(a,b,c, a*b*c, a*b*c+2*a+2*b+2*c-max(2*a,2*b,2*c))
		sum += a*b*c+2*a+2*b+2*c-max(2*a,2*b,2*c)
	}
	fmt.Println(sum)
}

