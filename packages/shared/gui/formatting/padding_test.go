package formatting

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestGetPadWidths(t *testing.T) {
	type scenario struct {
		input    [][]string
		expected []int
	}

	tests := []scenario{
		{
			[][]string{{""}, {""}},
			[]int{},
		},
		{
			[][]string{{"a"}, {""}},
			[]int{},
		},
		{
			[][]string{{"aa", "b", "ccc"}, {"c", "d", "e"}},
			[]int{2, 1},
		},
		{
			[][]string{{"AŁ", "b", "ccc"}, {"c", "d", "e"}},
			[]int{2, 1},
		},
	}

	for _, test := range tests {
		output := getPadWidths(test.input)
		assert.EqualValues(t, test.expected, output)
	}
}

func TestWithPadding(t *testing.T) {
	type scenario struct {
		str       string
		padding   int
		alignment Alignment
		expected  string
	}

	scenarios := []scenario{
		{
			str:       "hello world !",
			padding:   1,
			alignment: AlignLeft,
			expected:  "hello world !",
		},
		{
			str:       "hello world !",
			padding:   14,
			alignment: AlignLeft,
			expected:  "hello world ! ",
		},
		{
			str:       "hello world !",
			padding:   14,
			alignment: AlignRight,
			expected:  " hello world !",
		},
		{
			str:       "Güçlü",
			padding:   7,
			alignment: AlignLeft,
			expected:  "Güçlü  ",
		},
		{
			str:       "Güçlü",
			padding:   7,
			alignment: AlignRight,
			expected:  "  Güçlü",
		},
	}

	for _, s := range scenarios {
		assert.EqualValues(t, s.expected, WithPadding(s.str, s.padding, s.alignment))
	}
}
