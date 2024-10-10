package style

import (
	"github.com/gookit/color"
	"github.com/samber/lo"
)

func IsValidHexValue(v string) bool {
	if len(v) != 4 && len(v) != 7 {
		return false
	}

	if v[0] != '#' {
		return false
	}

	for _, char := range v[1:] {
		switch char {
		case '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C', 'D', 'E', 'F':
			continue
		default:
			return false
		}
	}

	return true
}

func SetCustomColors(customColors map[string]string) map[string]TextStyle {
	return lo.MapValues(customColors, func(c string, key string) TextStyle {
		if s, ok := ColorMap[c]; ok {
			return s.Foreground
		}
		return New().SetFg(NewRGBColor(color.HEX(c, false)))
	})
}
