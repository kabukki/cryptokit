package encoding

import "encoding/base64"

func Base64Encode(clear string) string {
	return base64.StdEncoding.EncodeToString([]byte(clear))
}

func Base64Decode(encoded string) string {
	decoded, _ := base64.StdEncoding.DecodeString(encoded)
	return string(decoded)
}
