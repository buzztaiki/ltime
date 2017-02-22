package main

import (
	"bufio"
	"bytes"
	"math/rand"
	"strconv"
	"testing"
)

func test(src string) string {
	b := bytes.Buffer{}
	filter(bytes.NewBufferString(src), &b)
	return string(b.Bytes())
}

func TestFilter(t *testing.T) {
	if test("") != "" {
		t.Error("empty bytes must be empty")
	}

	if x := test("2017-02-22T14:33:23.973Z"); x != "2017-02-22T23:33:23.973+09:00" {
		t.Error("RFC3339Nano string must be converted to local time", x)
	}

	if x := test("2017-02-22T14:33:23Z"); x != "2017-02-22T23:33:23+09:00" {
		t.Error("RFC3339 string must be converted to local time", x)
	}

	if x := test("2017-02-22T14:33:23.973"); x != "2017-02-22T14:33:23.973" {
		t.Error("time string without timezone must not be converted", x)
	}

	if x := test("2017/02/22 14:33:23"); x != "2017/02/22 14:33:23" {
		t.Error("unknown time string must not be converted", x)
	}

	b := bytes.Buffer{}
	for b.Len() < bufio.MaxScanTokenSize {
		b.WriteString(strconv.Itoa(rand.Int()))
	}

	s := string(b.Bytes())

	if x := test(s+"2017-02-22T14:33:23.973Z"); x != s+"2017-02-22T14:33:23.973Z" {
		t.Error("time string after long line must not be converted", "..." + x[len(x)-50:])
	}

	if x := test("2017-02-22T14:33:23.973Z"+s); x != "2017-02-22T23:33:23.973+09:00"+s {
		t.Error("time string before long line must be converted", x[:50] + "...")
	}
}
