package crypto

import (
	"crypto/md5"
	"encoding/hex"
	"math"
)

const CHARSET = "abcd"

func MD5Bruteforce(digest string) string {
	for length := 1; length <= 6; length++ {
		possibilities := int(math.Pow(float64(len(CHARSET)), float64(length)))
		println(length, possibilities)

		for n := 0; n < possibilities; n++ {
			// CHARSET[3]
			// md5.New().Write(['a']);
		}
	}

	return digest
}

func MD5Hash(clear string) string {
	digest := md5.Sum([]byte(clear))
	return hex.EncodeToString(digest[:])
}

// func Dictionary () {}

// func RainbowTables - stored somewhere
