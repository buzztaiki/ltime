package main

import (
	"os"
	"log"
)

func main() {
	if err := filter(os.Stdin, os.Stdout); err != nil {
		log.Fatalln(err)
	}
}
