package main

import (
	"fmt"

	"github.com/spf13/cobra"

	"hackit/crypto"
	"hackit/encoding"
)

func main() {
	app := &cobra.Command{
		Short: "Hacking toolkit",
	}

	// Base64
	cmdBase64 := &cobra.Command{
		Use:   "base64",
		Short: "Base64 encoding",
	}
	cmdBase64.AddCommand(&cobra.Command{
		Use:  "encode [clear]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(encoding.EncodeBase64(args[0]))
		},
	})
	cmdBase64.AddCommand(&cobra.Command{
		Use:  "decode [clear]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(encoding.DecodeBase64(args[0]))
		},
	})
	app.AddCommand(cmdBase64)

	// UU
	cmdUU := &cobra.Command{
		Use:   "uu",
		Short: "Unix to Unix encoding",
	}
	cmdUU.AddCommand(&cobra.Command{
		Use:  "encode [clear]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(encoding.EncodeUU(args[0]))
		},
	})
	cmdUU.AddCommand(&cobra.Command{
		Use:  "decode [encoded]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(encoding.DecodeUU(args[0]))
		},
	})
	app.AddCommand(cmdUU)

	// Hex
	cmdHex := &cobra.Command{
		Use:   "hex",
		Short: "Hexadecimal encoding",
	}
	cmdHex.AddCommand(&cobra.Command{
		Use:  "encode [clear]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(encoding.EncodeHex(args[0]))
		},
	})
	cmdHex.AddCommand(&cobra.Command{
		Use:  "decode [encoded]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(encoding.DecodeHex(args[0]))
		},
	})
	app.AddCommand(cmdHex)

	// MD5
	cmdMD5 := &cobra.Command{
		Use:   "md5",
		Short: "Message Digest 5",
	}
	cmdMD5.AddCommand(&cobra.Command{
		Use:  "bruteforce [digest]",
		Args: cobra.ExactArgs(1),
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Println(crypto.Bruteforce(args[0]))
		},
	})
	app.AddCommand(cmdMD5)

	app.Execute()
}
