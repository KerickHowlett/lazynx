package style

func (b TextStyle) SetBg(color Color) TextStyle {
	b.bg = &color
	b.Style = b.deriveStyle()
	return b
}

func (b TextStyle) SetFg(color Color) TextStyle {
	b.fg = &color
	b.Style = b.deriveStyle()
	return b
}
