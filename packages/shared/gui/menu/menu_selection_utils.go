package menu

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
