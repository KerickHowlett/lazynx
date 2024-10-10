package menu

import "packages/shared/gui/formatting"

type CreateMenuOptions struct {
	Title           string
	Prompt          string // a message that will be displayed above the menu options
	Items           []*MenuItem
	HideCancel      bool
	ColumnAlignment []formatting.Alignment
}

type MenuSection struct {
	Title  string
	Column int // The column that this section title should be aligned with
}

type MenuWidget int

const (
	MenuWidgetNone MenuWidget = iota
	MenuWidgetRadioButtonSelected
	MenuWidgetRadioButtonUnselected
	MenuWidgetCheckboxSelected
	MenuWidgetCheckboxUnselected
)
