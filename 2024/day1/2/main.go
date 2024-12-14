// Its slow but works
package main

import (
	"fmt"
	"log"
	"os"
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
	sum := 0
	for _,value := range distance {
		i := 0
		for _, v := range distance2 {
			if value == v {
					i += 1
			}
		}
		sum += value * i
	}
	fmt.Printf("%d", sum)
}

