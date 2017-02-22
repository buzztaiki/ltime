package main

import (
	"bufio"
	"bytes"
	"io"
	"regexp"
	"time"
)

func scanLines(data []byte, atEOF bool) (advance int, token []byte, err error) {
	if atEOF && len(data) == 0 {
		return 0, nil, nil
	}
	if i := bytes.IndexByte(data, '\n'); i >= 0 {
		return i + 1, data[0 : i+1], nil
	}
	if atEOF {
		return len(data), data, nil
	}
	// Request more data.
	return 0, nil, nil
}

func filter(r io.Reader, w io.Writer) error {
	scanner := bufio.NewScanner(r)
	scanner.Split(scanLines)
	re := regexp.MustCompile(`[\d-]+T[\d:]+\.\d+Z`)
	for scanner.Scan() {
		repl := re.ReplaceAllFunc(scanner.Bytes(), func(bs []byte) []byte {
			t, e := time.Parse(time.RFC3339Nano, string(bs))
			if e != nil {
				return bs
			} else {
				return []byte(t.Local().Format(time.RFC3339Nano))
			}
		})
		if _, err := w.Write(repl); err != nil {
			return err
		}
	}

	if err := scanner.Err(); err != nil {
		return err
	}

	return nil
}
