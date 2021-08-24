package encoding

import "encoding/hex"

func EncodeHex(clear string) string {
	return hex.EncodeToString([]byte(clear))
}

func DecodeHex(encoded string) string {
	decoded, _ := hex.DecodeString(encoded)
	return string(decoded)
}
