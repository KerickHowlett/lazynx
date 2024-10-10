package rendering

import "packages/shared/gui/context"

type MainContextPair struct {
	Main      context.Context
	Secondary context.Context
}

func NewMainContextPair(main context.Context, secondary context.Context) MainContextPair {
	return MainContextPair{Main: main, Secondary: secondary}
}
