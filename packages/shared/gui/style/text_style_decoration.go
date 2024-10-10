package style

func (b TextStyle) MergeStyle(other TextStyle) TextStyle {
	b.decoration = b.decoration.Merge(other.decoration)

	if other.fg != nil {
		b.fg = other.fg
	}

	if other.bg != nil {
		b.bg = other.bg
	}

	b.Style = b.deriveStyle()

	return b
}

// note that our receiver here is not a pointer which means we're receiving a
// copy of the original TextStyle. This allows us to mutate and return that
// TextStyle receiver without actually modifying the original.
func (b TextStyle) SetBold() TextStyle {
	b.decoration.SetBold()
	b.Style = b.deriveStyle()
	return b
}

func (b TextStyle) SetReverse() TextStyle {
	b.decoration.SetReverse()
	b.Style = b.deriveStyle()
	return b
}

func (b TextStyle) SetStrikethrough() TextStyle {
	b.decoration.SetStrikethrough()
	b.Style = b.deriveStyle()
	return b
}

func (b TextStyle) SetUnderline() TextStyle {
	b.decoration.SetUnderline()
	b.Style = b.deriveStyle()
	return b
}
