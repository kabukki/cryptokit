package cmd

import (
	"fmt"
	"hackit/crypto"

	"github.com/spf13/cobra"
)

func init() {
	md5 := &cobra.Command{
		Use:   "md5",
		Short: "Message Digest 5",
	}
	md5.AddCommand(&cobra.Command{
		Use:  "bruteforce [digest]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(crypto.MD5Bruteforce(args[0]))
		},
	})
	md5.AddCommand(&cobra.Command{
		Use:  "hash [clear]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(crypto.MD5Hash(args[0]))
		},
	})
	root.AddCommand(md5)
}
