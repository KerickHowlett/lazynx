package text

import (
	"github.com/gookit/color"

	c "packages/shared/gui/color"
)

// A TextStyle contains a foreground color, background color, and
// decorations (bold/underline/reverse).
//
// Colors may each be either 16-bit or 24-bit RGB colors. When
// we need to produce a string with a TextStyle, if either foreground or
// background color is RGB, we'll promote the other color component to RGB as well.
// We could simplify this code by forcing everything to be RGB, but we're not
// sure how compatible or efficient that would be with various terminals.
// LazyNx will typically stick to 16-bit colors, but users may configure RGB colors.
//
// TextStyles are value objects, not entities, so for example if you want to
// add the bold decoration to a TextStyle, we'll create a new TextStyle with
// that decoration applied.
//
// Decorations are additive, so when we merge two TextStyles, if either is bold
// then the resulting style will also be bold.
//
// So that we aren't re-deriving the underlying style each time we want to print
// a string, we derive it when a new TextStyle is created and store it in the
// `style` field.

type TextStyle struct {
	fg         *c.Color
	bg         *c.Color
	decoration Decoration

	// making this public so that we can use a type switch to get to the underlying
	// value so we can cache styles. This is very much a hack.
	Style Sprinter
}

func New() TextStyle {
	s := TextStyle{}
	s.Style = s.deriveStyle()
	return s
}

func (b TextStyle) deriveStyle() Sprinter {
	if b.fg == nil && b.bg == nil {
		return color.Style(b.decoration.ToOpts())
	}

	isRgb := (b.fg != nil && b.fg.IsRGB()) || (b.bg != nil && b.bg.IsRGB())
	if isRgb {
		return b.deriveRGBStyle()
	}

	return b.deriveBasicStyle()
}

func (b TextStyle) deriveBasicStyle() color.Style {
	style := make([]color.Color, 0, 5)

	if b.fg != nil {
		style = append(style, *b.fg.GetBasicColor())
	}

	if b.bg != nil {
		style = append(style, *b.bg.GetBasicColor())
	}

	style = append(style, b.decoration.ToOpts()...)

	return color.Style(style)
}

func (b TextStyle) deriveRGBStyle() *color.RGBStyle {
	style := &color.RGBStyle{}

	if b.fg != nil {
		style.SetFg(*b.fg.ToRGB(false).GetRGBColor())
	}

	if b.bg != nil {
		// We need to convert the bg firstly to a foreground color,
		// For more info see
		style.SetBg(*b.bg.ToRGB(true).GetRGBColor())
	}

	style.SetOpts(b.decoration.ToOpts())

	return style
}
