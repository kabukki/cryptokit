package cmd

import (
	"hackit/network"

	"github.com/spf13/cobra"
)

func init() {
	server := &cobra.Command{
		Use: "server",
	}

	serverListen := &cobra.Command{
		Use:  "listen",
		Args: cobra.NoArgs,
		Run: func(cmd *cobra.Command, args []string) {
			net, _ := cmd.Flags().GetString("type")
			host, _ := cmd.Flags().GetString("host")
			port, _ := cmd.Flags().GetString("port")

			network.ServerListen(net, host, port)
		},
	}
	serverListen.Flags().String("type", "tcp", "Network type")
	serverListen.Flags().String("host", "localhost", "Listening hostname")
	serverListen.Flags().String("port", "0", "Listening port")
	server.AddCommand(serverListen)

	root.AddCommand(server)
}
