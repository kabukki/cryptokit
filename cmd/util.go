package cmd

import (
	"fmt"

	"hackit/util"

	"github.com/spf13/cobra"
)

func init() {
	root.AddCommand(&cobra.Command{
		Use:  "strlen [input]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(util.StringLength(args[0]))
		},
	})
}
