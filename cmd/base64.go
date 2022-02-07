package cmd

import (
	"fmt"
	"hackit/encoding"

	"github.com/spf13/cobra"
)

func init() {
	base64 := &cobra.Command{
		Use:   "base64",
		Short: "Base64 encoding",
	}
	base64.AddCommand(&cobra.Command{
		Use:  "encode [clear]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(encoding.Base64Encode(args[0]))
		},
	})
	base64.AddCommand(&cobra.Command{
		Use:  "decode [clear]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(encoding.Base64Decode(args[0]))
		},
	})
	root.AddCommand(base64)
}
