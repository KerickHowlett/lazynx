package utils

import "github.com/jesseduffield/gocui"

func SafeWithError(f func() error) error {
	isPanicking := true
	defer func() {
		if isPanicking && gocui.Screen != nil {
			gocui.Screen.Fini()
		}
	}()

	err := f()

	isPanicking = false

	return err
}
