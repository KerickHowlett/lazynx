package text

import "packages/shared/gui/color"

func (b TextStyle) SetBg(color color.Color) TextStyle {
	b.bg = &color
	b.Style = b.deriveStyle()
	return b
}

func (b TextStyle) SetFg(color color.Color) TextStyle {
	b.fg = &color
	b.Style = b.deriveStyle()
	return b
}
