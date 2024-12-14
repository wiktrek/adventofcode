package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
    content, err := os.ReadFile("./2024/day2/1/input.txt")
	if err != nil {
        log.Fatal(err)
    }
    rows := strings.Split(string(content), "\r\n")
	levels := [1000][10]string{}
	for i,row := range rows {
		println(row)
		d := strings.Split(row, " ")
  		levels[i] = [10]string(d)
		if err != nil {
   	 	    log.Fatal(err)
  	 	}
    }
	safe := 0
	for _,l := range levels {
		safe_l := false
		for j,vs := range l {
			v, err := strconv.Atoi(vs)
			next, err := strconv.Atoi(l[j +1])
			if err	!= nil {
                log.Fatal(err)
           }
			decreasing := false
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
					fmt.Printf("%v", l)
					fmt.Println(decreasing)
					safe_l = false
				}
        }
		if safe_l == true {
			// fmt.Printf("%v\n",l)
			safe++
        }
		//  else if safe_l == false{
		// 	fmt.Printf("%v\n",l)
		// }
	}
	fmt.Printf("%d", safe)
}

