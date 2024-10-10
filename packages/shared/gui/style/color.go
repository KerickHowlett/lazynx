package style

import "github.com/gookit/color"

type Color struct {
	rgb   *color.RGBColor
	basic *color.Color
}

// @SECTION: BASIC Color

func (c Color) GetBasicColor() *color.Color {
	return c.basic
}

func NewBasicColor(cl color.Color) Color {
	c := Color{}
	c.basic = &cl
	return c
}

// @SECTION: RGB Color

func (c Color) GetRGBColor() *color.RGBColor {
	return c.rgb
}

func (c Color) IsRGB() bool {
	return c.rgb != nil
}

func NewRGBColor(cl color.RGBColor) Color {
	c := Color{}
	c.rgb = &cl
	return c
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
