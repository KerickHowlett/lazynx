package keybindings

import (
	"packages/shared/gui/types"

	"github.com/jesseduffield/gocui"
)

// Binding - a keybinding mapping a key and modifier to a handler. The keypress
// is only handled if the given view has focus, or handled globally if the view
// is ""
type Binding struct {
	ViewName    string
	Handler     func() error
	Key         Key
	Modifier    gocui.Modifier
	Description string
	// If defined, this is used in place of Description when showing the
	// keybinding in the options view at the bottom left of the screen.
	ShortDescription string
	Alternative      string
	// e.g. 'navigation'. Used for grouping things in the cheatsheet
	Tag       string
	OpensMenu bool

	// If true, the keybinding will appear at the bottom of the screen.
	//
	// Even if set to true, the keybinding will not be displayed if it is
	// currently disabled.
	//
	// We could instead display it with a strikethrough, but there's limited
	// real estate to show all the keybindings we want, so we're hiding it
	// instead.
	DisplayOnScreen bool

	// DisplayStyle *style.TextStyle

	// to be displayed if the keybinding is highlighted from within a menu
	Tooltip string

	// Function to decide whether the command is enabled, and why. If this
	// returns an empty string, it is; if it returns a non-empty string, it is
	// disabled and we show the given text in an error message when trying to
	// invoke it. When left nil, the command is always enabled. Note that this
	// function must not do expensive calls.
	GetDisabledReason func() *types.DisabledReason
}

func (b *Binding) IsDisabled() bool {
	return b.GetDisabledReason != nil && b.GetDisabledReason() != nil
}
