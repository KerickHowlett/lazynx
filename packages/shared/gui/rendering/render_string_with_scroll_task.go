package rendering

type RenderStringWithScrollTask struct {
	Str     string
	OriginX int
	OriginY int
}

func (t *RenderStringWithScrollTask) IsUpdateTask() {}

func NewRenderStringWithScrollTask(str string, originX int, originY int) *RenderStringWithScrollTask {
	return &RenderStringWithScrollTask{Str: str, OriginX: originX, OriginY: originY}
}
