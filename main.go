package main

import (
	"log"
	"os"
)

func main() {
	if err := filter(os.Stdin, os.Stdout); err != nil {
		log.Fatalln(err)
	}
}
