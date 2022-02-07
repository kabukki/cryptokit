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

func TestBase64Encode(t *testing.T) {
	for clear, expected := range testsBase64 {
		actual := Base64Encode(clear)
		if actual != expected {
			t.Fatalf(`"%s": Expected "%s" %v, actual "%s" %v`, clear, expected, []byte(expected), actual, []byte(actual))
		}
	}
}

func TestBase64Decode(t *testing.T) {
	for expected, encoded := range testsBase64 {
		actual := Base64Decode(encoded)
		if actual != expected {
			t.Fatalf(`"%s": Expected "%s" %v, actual "%s" %v`, encoded, expected, []byte(expected), actual, []byte(actual))
		}
	}
}
