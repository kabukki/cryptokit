package main

import (
	"fmt"
	"os"

	"hackit/cmd"

	"github.com/fatih/color"
)

func main() {
	defer func() {
		if err := recover(); err != nil {
			fmt.Fprintln(os.Stderr, color.RedString("%v", err))
			os.Exit(1)
		}
	}()

	cmd.Execute()
}
