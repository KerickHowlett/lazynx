package controller

import (
	"packages/shared/gui/keybindings"
	"packages/shared/gui/types"

	"github.com/jesseduffield/gocui"
)

type BaseController struct{}

func (bc *BaseController) GetKeybindings() []*keybindings.Binding {
	return nil
}

func (bc *BaseController) GetMouseKeybindings() *gocui.ViewMouseBinding {
	return nil
}

func (bc *BaseController) GetOnClick() func() error {
	return nil
}

func (bc *BaseController) GetOnRenderToMain() func() error {
	return nil
}

func (bc *BaseController) GetOnFocus() func(keybindings.KeybindingsOpts) error {
	return nil
}

func (bc *BaseController) GetOnBlur() func(types.OnBlurOpts) error {
	return nil
}
