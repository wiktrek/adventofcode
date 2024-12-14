package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
    content, err := os.ReadFile("./2024/day2/input.txt")
	if err != nil {
        log.Fatal(err)
    }
    rows := strings.Split(string(content), "\r\n")
	levels := [1000][10]string{}
	for i,row := range rows {
		// println(row)
		d := strings.Split(row, " ")
		copy(levels[i][:], d[:])
		if err != nil {
   	 	    log.Fatal(err)
  	 	}
    }
	safe := 0
	for _,l := range levels {
		if check_correct(l) {
			safe++
		}
	}
	fmt.Printf("%d", safe)
}
func check_correct(l [10]string) bool {
	safe_l := false
		decreasing := false
		for j,vs := range l {
			v := convert(vs)
			next := convert(l[j+1])
		   
				if j == 0 {
					if v < next {
						decreasing = false
					} else {
						decreasing = true
					}
				}
				if v == 0 || next == 0 {
					break
				}
				if decreasing  == true && 0 < v - next && v-next <= 3{
					safe_l = true
				} else if decreasing == false && 0 < next - v && next - v <= 3 {
					safe_l = true
				} else {
					safe_l = false
					break
				}
        }
		return safe_l
}
func convert(str string) int {
           if str == "" {
               return 0
           }
			v, err := strconv.Atoi(str)
			if err	!= nil {
                log.Fatal(err)
           }	
		   return v
} 