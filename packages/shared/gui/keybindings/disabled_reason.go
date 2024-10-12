package keybindings

type DisabledReason struct {
	Text string

	// When trying to invoke a disabled key binding or menu item, we normally
	// show the disabled reason as a toast; setting this to true shows it as an
	// error panel instead. This is useful if the text is very long, or if it is
	// important enough to show it more prominently, or both.
	ShowErrorInPanel bool
}
