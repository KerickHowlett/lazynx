package menu

import (
	"packages/shared/gui/keybindings"
	"packages/shared/gui/types"
)

type MenuItem struct {
	Label string

	// alternative to Label. Allows specifying columns which will be auto-aligned
	LabelColumns []string

	OnPress func() error

	// Only applies when Label is used
	OpensMenu bool

	// If Key is defined it allows the user to press the key to invoke the menu
	// item, as opposed to having to navigate to it
	Key keybindings.Key

	// A widget to show in front of the menu item. Supported widget types are
	// checkboxes and radio buttons,
	// This only handles the rendering of the widget; the behavior needs to be
	// provided by the client.
	Widget MenuWidget

	// The tooltip will be displayed upon highlighting the menu item
	Tooltip string

	// If non-nil, show this in a tooltip, style the menu item as disabled,
	// and refuse to invoke the command
	DisabledReason *types.DisabledReason

	// Can be used to group menu items into sections with headers. MenuItems
	// with the same Section should be contiguous, and will automatically get a
	// section header. If nil, the item is not part of a section.
	// Note that pointer comparison is used to determine whether two menu items
	// belong to the same section, so make sure all your items in a given
	// section point to the same MenuSection instance.
	Section *MenuSection
}

// Defining this for the sake of conforming to the HasID interface, which is used
// in list contexts.
func (mi *MenuItem) ID() string {
	return mi.Label
}
