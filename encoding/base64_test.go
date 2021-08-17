package encoding

import "testing"

var testsBase64 = map[string]string{
	"":            "",
	"H":           "SA==",
	"He":          "SGU=",
	"Hel":         "SGVs",
	"Hell":        "SGVsbA==",
	"Hello":       "SGVsbG8=",
	"Hello ":      "SGVsbG8g",
	"Hello W":     "SGVsbG8gVw==",
	"Hello Wo":    "SGVsbG8gV28=",
	"Hello Wor":   "SGVsbG8gV29y",
	"Hello Worl":  "SGVsbG8gV29ybA==",
	"Hello World": "SGVsbG8gV29ybGQ=",
}

func TestEncodeBase64(t *testing.T) {
	for clear, expected := range testsBase64 {
		actual := EncodeBase64(clear)
		if actual != expected {
			t.Fatalf(`"%s": Expected "%s" %v, actual "%s" %v`, clear, expected, []byte(expected), actual, []byte(actual))
		}
	}
}

func TestDecodeBase64(t *testing.T) {
	for expected, encoded := range testsBase64 {
		actual := DecodeBase64(encoded)
		if actual != expected {
			t.Fatalf(`"%s": Expected "%s" %v, actual "%s" %v`, encoded, expected, []byte(expected), actual, []byte(actual))
		}
	}
}
