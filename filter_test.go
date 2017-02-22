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
		t.Fatal("empty bytes must be empty")
	}

	if test("2017-02-22T14:33:23.973Z") != "2017-02-22T23:33:23.973+09:00" {
		t.Fatal("RFC3339Nano string must be converted to local time")
	}

	if test("2017-02-22T14:33:23.973") != "2017-02-22T14:33:23.973" {
		t.Fatal("time string without timezone must not be converted")
	}

	if test("2017/02/22 14:33:23") != "2017/02/22 14:33:23" {
		t.Fatal("unknown time string must not be converted")
	}

	b := bytes.Buffer{}
	for b.Len() < bufio.MaxScanTokenSize {
		b.WriteString(strconv.Itoa(rand.Int()))
	}

	s := string(b.Bytes())

	if test(s+"2017-02-22T14:33:23.973Z") != s+"2017-02-22T14:33:23.973Z" {
		t.Fatal("time string after long line must not be converted")
	}

	if test("2017-02-22T14:33:23.973Z"+s) != "2017-02-22T23:33:23.973+09:00"+s {
		t.Fatal("time string before long line must be converted")
	}
}
