package cmd

import (
	"fmt"

	"hackit/format"

	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

func init() {
	uu := &cobra.Command{
		Use:   "json",
		Short: "JavaScript Object Notation",
	}
	uu.AddCommand(&cobra.Command{
		Use:  "compact [json]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(format.JSONCompact(args[0]))
		},
	})
	uu.AddCommand(&cobra.Command{
		Use:  "pretty [json]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(format.JSONPretty(args[0]))
		},
	})
	uu.AddCommand(&cobra.Command{
		Use:  "test [json]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			valid := format.JSONTest(args[0])
			if valid {
				fmt.Println("JSON is", color.GreenString("valid"), "!")
			} else {
				fmt.Println("JSON is", color.RedString("invalid"), "!")
			}
		},
	})
	root.AddCommand(uu)
}
