package keybindings

import "github.com/jesseduffield/gocui"

type HasKeybindings interface {
	GetKeybindings(opts KeybindingsOpts) []*Binding
	GetMouseKeybindings(opts KeybindingsOpts) []*gocui.ViewMouseBinding
	GetOnClick() func() error
	GetOnRenderToMain() func() error
	GetOnFocus() func(OnFocusOpts) error
	GetOnBlur() func(OnBlurOpts) error
}

type (
	KeybindingsFn      func(opts KeybindingsOpts) []*Binding
	MouseKeybindingsFn func(opts KeybindingsOpts) []*gocui.ViewMouseBinding
)
