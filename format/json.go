package format

import (
	"bytes"
	"encoding/json"
)

func JSONPretty(input string) string {
	var out bytes.Buffer

	err := json.Indent(&out, []byte(input), "", "  ")
	if err != nil {
		panic(err)
	}

	return out.String()
}

func JSONCompact(input string) string {
	var out bytes.Buffer

	err := json.Compact(&out, []byte(input))
	if err != nil {
		panic(err)
	}

	return out.String()
}

func JSONTest(input string) bool {
	return json.Valid([]byte(input))
}
