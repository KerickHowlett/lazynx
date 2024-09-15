package status

import (
	"packages/shared/types"
)

type StatusController struct{}

var _ types.IController = &StatusController{}

func NewStatusController() *StatusController {
	return &StatusController{}
}
