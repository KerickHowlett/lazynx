package status

import (
	"packages/shared/gui/controller"
)

type StatusController struct {
	controller.BaseController
}

var _ controller.IController = &StatusController{}

func NewStatusController() *StatusController {
	return &StatusController{}
}
