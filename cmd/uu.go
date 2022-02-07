package cmd

import (
	"fmt"
	"hackit/encoding"

	"github.com/spf13/cobra"
)

func init() {
	uu := &cobra.Command{
		Use:   "uu",
		Short: "Unix to Unix encoding",
	}
	uu.AddCommand(&cobra.Command{
		Use:  "encode [clear]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(encoding.UUEncode(args[0]))
		},
	})
	uu.AddCommand(&cobra.Command{
		Use:  "decode [encoded]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(encoding.UUDecode(args[0]))
		},
	})
	root.AddCommand(uu)
}
