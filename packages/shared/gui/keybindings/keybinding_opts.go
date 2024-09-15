package keybindings

type KeybindingsOpts struct {
	GetKey func(key string) Key
	Guards KeybindingGuards
}
