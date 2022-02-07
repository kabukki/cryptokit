package encoding

import "testing"

var testsUU = map[string]string{
	"H":           "!2```",
	"He":          "\"2&4`",
	"Hel":         "#2&5L",
	"Hell":        "$2&5L;```",
	"Hello":       "%2&5L;&\\`",
	"Hello ":      "&2&5L;&\\@",
	"Hello W":     "'2&5L;&\\@5P``",
	"Hello Wo":    "(2&5L;&\\@5V\\`",
	"Hello Wor":   ")2&5L;&\\@5V]R",
	"Hello Worl":  "*2&5L;&\\@5V]R;```",
	"Hello World": "+2&5L;&\\@5V]R;&0`",
}

func TestUUEncode(t *testing.T) {
	for clear, expected := range testsUU {
		actual := UUEncode(clear)
		if actual != expected {
			t.Fatalf(`"%s": Expected "%s" %v, actual "%s" %v`, clear, expected, []byte(expected), actual, []byte(actual))
		}
	}
}

func TestUUDecode(t *testing.T) {
	for expected, encoded := range testsUU {
		actual := UUDecode(encoded)
		if actual != expected {
			t.Fatalf(`"%s": Expected "%s" %v, actual "%s" %v`, encoded, expected, []byte(expected), actual, []byte(actual))
		}
	}
}
