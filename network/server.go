package network

import (
	"fmt"
	"io"
	"net"
)

func ServerListen(network string, host string, port string) {
	address := net.JoinHostPort(host, port)
	server, err := net.Listen(network, address)
	if err != nil {
		panic(err)
	}
	defer server.Close()

	fmt.Println("Listening on", server.Addr().String())

	connection, err := server.Accept()
	if err != nil {
		panic(err)
	}
	defer connection.Close()

	buf := make([]byte, 1024)
	read := 0
	fmt.Println("Connection from", connection.RemoteAddr().String())

	for {
		bytes, err := connection.Read(buf)
		read += bytes
		if err == io.EOF {
			break
		} else if err != nil {
			panic(err)
		}

		fmt.Println(string(buf))
	}

	fmt.Println("Connection closed, read", read, "bytes in total")
}
