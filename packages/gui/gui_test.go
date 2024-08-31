package gui

import (
	"testing"
)

func TestGui(t *testing.T) {
	result := Gui("works")
	if result != "Gui works" {
		t.Error("Expected Gui to append 'works'")
	}
}
