package color

import (
	"regexp"
	"sync"

	"github.com/gookit/color"
)

type Color struct {
	rgb   *color.RGBColor
	basic *color.Color
}

var (
	decolorizeCache = make(map[string]string)
	decolorizeMutex sync.RWMutex
)

// Decolorize strips a string of color
func Decolorize(str string) string {
	decolorizeMutex.RLock()
	val := decolorizeCache[str]
	decolorizeMutex.RUnlock()

	if val != "" {
		return val
	}

	re := regexp.MustCompile(`\x1B\[([0-9]{1,3}(;[0-9]{1,3})*)?[mGK]`)
	ret := re.ReplaceAllString(str, "")

	decolorizeMutex.Lock()
	decolorizeCache[str] = ret
	decolorizeMutex.Unlock()

	return ret
}

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

func NewRGBColor(cl color.RGBColor) Color {
	c := Color{}
	c.rgb = &cl
	return c
}

func NewBasicColor(cl color.Color) Color {
	c := Color{}
	c.basic = &cl
	return c
}

func (c Color) IsRGB() bool {
	return c.rgb != nil
}

func (c Color) ToRGB(isBg bool) Color {
	if c.IsRGB() {
		return c
	}

	if isBg {
		// We need to convert bg color to fg color
		// This is a gookit/color bug,
		// https://github.com/gookit/color/issues/39
		return NewRGBColor((*c.basic - 10).RGB())
	}

	return NewRGBColor(c.basic.RGB())
}

// func SetCustomColors(customColors map[string]string) map[string]style.TextStyle {
// 	return lo.MapValues(customColors, func(c string, key string) style.TextStyle {
// 		if s, ok := style.ColorMap[c]; ok {
// 			return s.Foreground
// 		}
// 		return style.New().SetFg(style.NewRGBColor(color.HEX(c, false)))
// 	})
// }
