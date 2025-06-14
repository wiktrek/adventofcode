package main

import (
	"crypto/md5"
	"encoding/hex"
	"os"
	"strconv"
)
func check(e error) {
    if e != nil {
        panic(e)
    }
}
func main() {
	data,err := os.ReadFile("./input.txt")

	check(err)
	two_parts(string(data), 5) // part 1
	two_parts(string(data), 6) // part 2
}

func two_parts(input string, zeros int) {
	t := false
	for i:=0;!t;i++ {
		f := input + strconv.Itoa(i)
		hex := GetMD5Hash(f)
		for j:=0;j<zeros;j++ {
			if hex[j] != '0' {
				break
			}
			if j == zeros - 1 {
				println(i)
				t = true
			}
		}
	}
}
func GetMD5Hash(text string) string {
   hash := md5.Sum([]byte(text))
   return hex.EncodeToString(hash[:])
}

