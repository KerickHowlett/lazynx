package status

import (
	"packages/shared/gui/controller"
)

type StatusController struct{}

var _ controller.IController = &StatusController{}

func NewStatusController() *StatusController {
	return &StatusController{}
}
