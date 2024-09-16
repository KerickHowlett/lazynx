package helpers

import "packages/shared/gui/context"

type HelperCommon struct {
	IGetContextTree
}

type IGetContextTree interface {
	ContextTree() *context.ContextTree
}
