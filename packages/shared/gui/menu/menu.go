package menu

type CreateMenuOptions struct {
	Title      string
	Prompt     string // a message that will be displayed above the menu options
	Items      []*MenuItem
	HideCancel bool
	// ColumnAlignment []utils.Alignment
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

func MakeMenuRadioButton(value bool) MenuWidget {
	if value {
		return MenuWidgetRadioButtonSelected
	}
	return MenuWidgetRadioButtonUnselected
}

func MakeMenuCheckBox(value bool) MenuWidget {
	if value {
		return MenuWidgetCheckboxSelected
	}
	return MenuWidgetCheckboxUnselected
}
