package encoding

import "encoding/hex"

func HexEncode(clear string) string {
	return hex.EncodeToString([]byte(clear))
}

func HexDecode(encoded string) string {
	decoded, _ := hex.DecodeString(encoded)
	return string(decoded)
}
