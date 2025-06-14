package main

import (
	"fmt"
	"os"
	"regexp"
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
	nice_strings := 0
	bad_letters := []string{"ab","cd","pq","xy"}
	for i:=0;i<len(lines);i++ {
		b:= false;
		for j:=0;j<len(bad_letters);j++ {
			if strings.Contains(lines[i], bad_letters[j]) {
				b = true
				break
			}
		}
		if b {
			continue
		}
		r := regexp.MustCompile("[aeiou]").FindAllString(lines[i],-1)
		if len(r) < 3 {
			continue
		}
		a:=false
		for j:=0;j<len(lines[i]) - 1;j++{
			if lines[i][j] == lines[i][j+1] {
				// println(lines[i])
				a=true;
				break
			}
		}
		if a && !b {
			nice_strings++
		}
	}
	println(nice_strings)
}

func part2(input string) {
	lines := strings.Split(input, "\n")
	nice_strings := 0
	for i := range lines  {
		a:= false
		for j:=0;j<len(lines[i])-2;j++ {
			if lines[i][j] == lines[i][j+2] {
				a = true
				break
			}
		}
		if !a {
			continue
		}
		b:=false;
		for j:=0;j<len(lines[i])-1;j++ {
			two_chars:= fmt.Sprintf("%c%c",lines[i][j], lines[i][j+1])
			for k:=j+2;k<len(lines[i])-1;k++ {
				two_chars2 := fmt.Sprintf("%c%c",lines[i][k], lines[i][k+1])
				if two_chars == two_chars2 {
					b = true
					break
				}
			}
		}
		if !b {
			continue
		}
		nice_strings++
	}
	
	println(nice_strings)
}


