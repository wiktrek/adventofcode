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
	floor:= 0;
	check(err)
	for i:=0;i<len(data);i++ {
		if data[i] == '(' {
			floor +=1
		} else {
			floor -= 1
		}
	}
	fmt.Print(floor)
}