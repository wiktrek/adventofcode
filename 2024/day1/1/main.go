package main

import (
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
    content, err := os.ReadFile("./2024/day1/1/input.txt")
	distance := [1000]int{}
	distance2 := [1000]int{}
	if err != nil {
        log.Fatal(err)
    }
    rows := strings.Split(string(content), "\r\n")
	for i,row := range rows {
		println(row)
		d := strings.Split(row, "   ")
		distance[i], err = strconv.Atoi(d[0])
		distance2[i], err = strconv.Atoi(d[1])
		if err != nil {
   	 	    log.Fatal(err)
  	 	}
    }
	sort.Ints(distance[:])
	sort.Ints(distance2[:])
	sum := 0;
	for i := 0; i < 1000; i++ {
		if distance[i] > distance2[i] {
			sum += distance[i] - distance2[i]
		} else {
			sum += distance2[i] - distance[i]
		}
	} 
	fmt.Println(sum)
}
