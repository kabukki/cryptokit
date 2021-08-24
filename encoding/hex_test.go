package encoding

import "testing"

var testsHex = map[string]string{
	"H":           "48",
	"He":          "4865",
	"Hel":         "48656c",
	"Hell":        "48656c6c",
	"Hello":       "48656c6c6f",
	"Hello ":      "48656c6c6f20",
	"Hello W":     "48656c6c6f2057",
	"Hello Wo":    "48656c6c6f20576f",
	"Hello Wor":   "48656c6c6f20576f72",
	"Hello Worl":  "48656c6c6f20576f726c",
	"Hello World": "48656c6c6f20576f726c64",
}

func TestEncodeHex(t *testing.T) {
	for clear, expected := range testsHex {
		actual := EncodeHex(clear)
		if actual != expected {
			t.Fatalf(`"%s": Expected "%s" %v, actual "%s" %v`, clear, expected, []byte(expected), actual, []byte(actual))
		}
	}
}

func TestDecodeHex(t *testing.T) {
	for expected, encoded := range testsHex {
		actual := DecodeHex(encoded)
		if actual != expected {
			t.Fatalf(`"%s": Expected "%s" %v, actual "%s" %v`, encoded, expected, []byte(expected), actual, []byte(actual))
		}
	}
}
