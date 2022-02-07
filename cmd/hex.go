package cmd

import (
	"fmt"
	"hackit/encoding"

	"github.com/spf13/cobra"
)

func init() {
	hex := &cobra.Command{
		Use:   "hex",
		Short: "Hexadecimal encoding",
	}
	hex.AddCommand(&cobra.Command{
		Use:  "encode [clear]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(encoding.HexEncode(args[0]))
		},
	})
	hex.AddCommand(&cobra.Command{
		Use:  "decode [encoded]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(encoding.HexDecode(args[0]))
		},
	})
	root.AddCommand(hex)
}
