package controller

type ControllerCommon struct {
	IGetHelpers
}

type IGetHelpers interface {
	Helpers() any // TODO: Change return type to `*helpers.Helpers`
}

func NewControllerCommon() *ControllerCommon {
	return &ControllerCommon{}
}
