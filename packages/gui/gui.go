package gui

import (
	"github.com/jesseduffield/gocui"
)

type GUI struct {
	g *gocui.Gui
}

func (gui *GUI) Run() error {
	g, err := gui.initGoCUI()
	if err != nil {
		return err
	}

	gui.g = g

	return nil
}

func (gui *GUI) initGoCUI() (g *gocui.Gui, err error) {
	g, err = gocui.NewGui(gocui.NewGuiOpts{
		OutputMode:      gocui.OutputTrue,
		SupportOverlaps: false,
		PlayRecording:   false,
		Headless:        false,
		Width:           0,
		Height:          0,
	})

	if err != nil {
		return nil, err
	}

	gui.g = g
	return g, nil
}

func NewGUI() (gui *GUI) {
	return &GUI{}
}
