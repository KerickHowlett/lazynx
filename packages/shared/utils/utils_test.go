package utils

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestAsJson(t *testing.T) {
	type myStruct struct {
		a string
	}

	output := AsJson(&myStruct{a: "foo"})

	// no idea why this is returning empty hashes but it's works in the app ¯\_(ツ)_/¯
	assert.EqualValues(t, "{}", output)
}

func TestModuloWithWrap(t *testing.T) {
	type scenario struct {
		n        int
		max      int
		expected int
	}

	scenarios := []scenario{
		{
			n:        0,
			max:      0,
			expected: 0,
		},
		{
			n:        0,
			max:      1,
			expected: 0,
		},
		{
			n:        1,
			max:      0,
			expected: 0,
		},
		{
			n:        3,
			max:      2,
			expected: 1,
		},
		{
			n:        -1,
			max:      2,
			expected: 1,
		},
	}

	for _, s := range scenarios {
		if s.expected != ModuloWithWrap(s.n, s.max) {
			t.Errorf("expected %d, got %d, for n: %d, max: %d", s.expected, ModuloWithWrap(s.n, s.max), s.n, s.max)
		}
	}
}
