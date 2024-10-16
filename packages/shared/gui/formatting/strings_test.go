package formatting

import (
	"strings"
	"testing"

	"github.com/mattn/go-runewidth"
	"github.com/stretchr/testify/assert"
)

func TestSafeTruncate(t *testing.T) {
	type scenario struct {
		str      string
		limit    int
		expected string
	}

	scenarios := []scenario{
		{
			str:      "",
			limit:    0,
			expected: "",
		},
		{
			str:      "12345",
			limit:    3,
			expected: "123",
		},
		{
			str:      "12345",
			limit:    4,
			expected: "1234",
		},
		{
			str:      "12345",
			limit:    5,
			expected: "12345",
		},
		{
			str:      "12345",
			limit:    6,
			expected: "12345",
		},
	}

	for _, s := range scenarios {
		assert.EqualValues(t, s.expected, SafeTruncate(s.str, s.limit))
	}
}

func TestTruncateWithEllipsis(t *testing.T) {
	// will need to check chinese characters as well
	// important that we have a three dot ellipsis within the limit
	type scenario struct {
		str      string
		limit    int
		expected string
	}

	scenarios := []scenario{
		{
			"hello world !",
			1,
			".",
		},
		{
			"hello world !",
			2,
			"..",
		},
		{
			"hello world !",
			3,
			"he…",
		},
		{
			"hello world !",
			4,
			"hel…",
		},
		{
			"hello world !",
			5,
			"hell…",
		},
		{
			"hello world !",
			12,
			"hello world…",
		},
		{
			"hello world !",
			13,
			"hello world !",
		},
		{
			"hello world !",
			14,
			"hello world !",
		},
		{
			"大大大大",
			5,
			"大大…",
		},
		{
			"大大大大",
			2,
			"..",
		},
		{
			"大大大大",
			1,
			".",
		},
		{
			"大大大大",
			0,
			"",
		},
	}

	for _, s := range scenarios {
		assert.EqualValues(t, s.expected, TruncateWithEllipsis(s.str, s.limit))
	}
}

func TestRenderDisplayStrings(t *testing.T) {
	type scenario struct {
		input                   [][]string
		columnAlignments        []Alignment
		expectedOutput          string
		expectedColumnPositions []int
	}

	tests := []scenario{
		{
			input:                   [][]string{{""}, {""}},
			columnAlignments:        nil,
			expectedOutput:          "",
			expectedColumnPositions: []int{0, 0},
		},
		{
			input:                   [][]string{{"a"}, {""}},
			columnAlignments:        nil,
			expectedOutput:          "a\n",
			expectedColumnPositions: []int{0},
		},
		{
			input:                   [][]string{{"a"}, {"b"}},
			columnAlignments:        nil,
			expectedOutput:          "a\nb",
			expectedColumnPositions: []int{0},
		},
		{
			input:                   [][]string{{"a", "b"}, {"c", "d"}},
			columnAlignments:        nil,
			expectedOutput:          "a b\nc d",
			expectedColumnPositions: []int{0, 2},
		},
		{
			input:                   [][]string{{"a", "", "c"}, {"d", "", "f"}},
			columnAlignments:        nil,
			expectedOutput:          "a c\nd f",
			expectedColumnPositions: []int{0, 2, 2},
		},
		{
			input:                   [][]string{{"a", "", "c", ""}, {"d", "", "f", ""}},
			columnAlignments:        nil,
			expectedOutput:          "a c\nd f",
			expectedColumnPositions: []int{0, 2, 2},
		},
		{
			input:                   [][]string{{"abc", "", "d", ""}, {"e", "", "f", ""}},
			columnAlignments:        nil,
			expectedOutput:          "abc d\ne   f",
			expectedColumnPositions: []int{0, 4, 4},
		},
		{
			input:                   [][]string{{"", "abc", "", "", "d", "e"}, {"", "f", "", "", "g", "h"}},
			columnAlignments:        nil,
			expectedOutput:          "abc d e\nf   g h",
			expectedColumnPositions: []int{0, 0, 4, 4, 4, 6},
		},
		{
			input:                   [][]string{{"abc", "", "d", ""}, {"e", "", "f", ""}},
			columnAlignments:        []Alignment{AlignLeft, AlignLeft}, // same as nil (default)
			expectedOutput:          "abc d\ne   f",
			expectedColumnPositions: []int{0, 4, 4},
		},
		{
			input:                   [][]string{{"abc", "", "d", ""}, {"e", "", "f", ""}},
			columnAlignments:        []Alignment{AlignRight, AlignLeft},
			expectedOutput:          "abc d\n  e f",
			expectedColumnPositions: []int{0, 4, 4},
		},
		{
			input:                   [][]string{{"a", "", "bcd", "efg", "h"}, {"i", "", "j", "k", "l"}},
			columnAlignments:        []Alignment{AlignLeft, AlignLeft, AlignRight, AlignLeft},
			expectedOutput:          "a bcd efg h\ni   j k   l",
			expectedColumnPositions: []int{0, 2, 2, 6, 10},
		},
		{
			input:                   [][]string{{"abc", "", "d", ""}, {"e", "", "f", ""}},
			columnAlignments:        []Alignment{AlignRight}, // gracefully defaults unspecified columns to left-align
			expectedOutput:          "abc d\n  e f",
			expectedColumnPositions: []int{0, 4, 4},
		},
	}

	for _, test := range tests {
		output, columnPositions := RenderDisplayStrings(test.input, test.columnAlignments)
		assert.EqualValues(t, test.expectedOutput, strings.Join(output, "\n"))
		assert.EqualValues(t, test.expectedColumnPositions, columnPositions)
	}
}

func BenchmarkStringWidthAsciiOriginal(b *testing.B) {
	for i := 0; i < b.N; i++ {
		runewidth.StringWidth("some ASCII string")
	}
}

func BenchmarkStringWidthAsciiOptimized(b *testing.B) {
	for i := 0; i < b.N; i++ {
		StringWidth("some ASCII string")
	}
}

func BenchmarkStringWidthNonAsciiOriginal(b *testing.B) {
	for i := 0; i < b.N; i++ {
		runewidth.StringWidth("some non-ASCII string 🍉")
	}
}

func BenchmarkStringWidthNonAsciiOptimized(b *testing.B) {
	for i := 0; i < b.N; i++ {
		StringWidth("some non-ASCII string 🍉")
	}
}
