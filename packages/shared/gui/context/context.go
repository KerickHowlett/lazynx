package context

import "packages/shared/gui/keybindings"

type Context interface {
	IBaseContext

	HandleBlur(opts keybindings.OnBlurOpts) error
	HandleFocus(opts keybindings.OnFocusOpts) error
	HandleRender() error
	HandleRenderToMain() error
}
