package encoding

import "encoding/base64"

func EncodeBase64(clear string) string {
	return base64.StdEncoding.EncodeToString([]byte(clear))
}

func DecodeBase64(encoded string) string {
	decoded, _ := base64.StdEncoding.DecodeString(encoded)
	return string(decoded)
}
