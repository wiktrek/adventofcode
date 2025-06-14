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
type House struct {
	x,y, presents int
}
func part1(input string) {
	sum :=0
	var presents []House
	var x,y int
	for i:=0;i<len(input);i++ {
		if i == 0 {
			presents = append(presents, House {
				x:0,
				y:0,
				presents: 1,
			})
		}
		char:= input[i]
		if char == '>' {
			x++
		}
		if char == '<' {
			x--

		}
		if char == '^' {
			y++
		}
		if char == 'v' {
			y--
		}
		a := false;
		for i:=0;i<len(presents);i++ {
			if presents[i].x == x && presents[i].y == y {
				presents[i].presents += 1
				a = true
			}
		}
		if !a {
			presents = append(presents, House{
				x: x,
				y: y,
				presents: 1,
			})
		}
	}
	for i:=0;i<len(presents);i++ {
		if presents[i].presents >= 1 {
			sum += 1
		}
	}
	fmt.Println(presents)
	fmt.Println(sum)
}

func part2(input string) {
	sum :=0
	var presents []House
	var x,y int
	var robo_x, robo_y int
	for i:=0;i<len(input);i++ {
		if i == 0 {
			presents = append(presents, House {
				x:0,
				y:0,
				presents: 1,
			})
		}
		char:= input[i]
		if char == '>' {
			if i % 2 == 0 {
				robo_x++
			} else {
				x++
			}	
		}
		if char == '<' {
			if i % 2 == 0 {
				robo_x--
			} else {
				x--
			}
		}
		if char == '^' {
			if i % 2 == 0 {
				robo_y++
			} else {
				y++
			}
		}
		if char == 'v' {
			if i % 2 == 0 {
				robo_y--
			} else {
				y--
			}
		}
		a := false;
		for j:=0;j<len(presents);j++ {
			if (i % 2 == 1 && presents[j].x == x && presents[j].y == y) || (i % 2 == 0 && presents[j].x == robo_x && presents[j].y == robo_y) {
				presents[j].presents += 1
				a = true
			}
		}
		if !a {
			if i % 2 == 0 {
				presents = append(presents, House{
					x: robo_x,
					y: robo_y,
					presents: 1,
				})
			} else {
				presents = append(presents, House{
					x: x,
					y: y,
					presents: 1,
				})
			}

		}
	}
	for i:=0;i<len(presents);i++ {
		if presents[i].presents >= 1 {
			sum += 1
		}
	}
	fmt.Println(presents)
	fmt.Println(sum)
}

