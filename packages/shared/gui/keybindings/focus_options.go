package keybindings

type ContextKey string

type OnBlurOpts struct {
	NewContextKey ContextKey
}

type OnFocusOpts struct {
	ClickedWindowName  string
	ClickedViewLineIdx int
}
