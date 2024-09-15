package gui

import (
	"packages/shared/utils"

	"github.com/jesseduffield/gocui"
)

type GUI struct {
	g        *gocui.Gui
	stopChan chan struct{}
}

func (gui *GUI) Run() error {
	g, err := gui.initGoCUI()
	if err != nil {
		return err
	}

	gui.g = g
	defer gui.g.Close()

	// TODO: Toggle setting with customizable config file.
	gui.g.Mouse = true

	return gui.g.MainLoop()
}

func (gui *GUI) RunAndHandleError() error {
	gui.stopChan = make(chan struct{})

	return utils.SafeWithError(func() error {
		if err := gui.Run(); err != nil {
			close(gui.stopChan)
			return gui.determineRunError(err)
		}

		return nil
	})
}

func (gui *GUI) determineRunError(err error) error {
	switch err {
	case gocui.ErrQuit:
		return nil
	default:
		return err
	}
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
