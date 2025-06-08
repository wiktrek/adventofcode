package main

import (
	"fmt"
	"os"
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
func part2(input string) {
	floor:= 0;
	for i:=0;i<len(input);i++ {
		if input[i] == '(' {
			floor +=1
		} else {
			floor -= 1
		}
		if floor == -1 {
			fmt.Println(i+1)
			return
		}
	}
}

func part1(input string) {
	floor:= 0;
	
	for i:=0;i<len(input);i++ {
		if input[i] == '(' {
			floor +=1
		} else {
			floor -= 1
		}

	}
	fmt.Println(floor)
}