package encoding

import (
	"encoding/base64"
	"strings"
)

const CHARSET = "`!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_"

func EncodeUU(clear string) string {
	encoding := base64.NewEncoding(CHARSET).WithPadding(' ')

	encoded := make([]byte, encoding.EncodedLen(len(clear)))
	encoding.Encode(encoded, []byte(clear))
	encoded = append([]byte{byte(len(clear) + 32)}, encoded...)

	return strings.ReplaceAll(string(encoded), " ", "`")
}

func DecodeUU(encoded string) string {
	length := encoded[0] - 32
	fixed := []byte(encoded[1:])
	encoding := base64.NewEncoding(CHARSET).WithPadding(' ')

	// Use new padding character
	for n := len(fixed) - 1; n > 0; n-- {
		if fixed[n] == '`' {
			fixed[n] = ' '
		} else {
			break
		}
	}

	decoded := make([]byte, encoding.DecodedLen(len(fixed)))
	encoding.Decode(decoded, fixed)

	return string(decoded[:length])
}
