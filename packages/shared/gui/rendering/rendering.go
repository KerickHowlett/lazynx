package rendering

type RefreshMainOpts struct {
	Pair      MainContextPair
	Main      *ViewUpdateOpts
	Secondary *ViewUpdateOpts
}

type UpdateTask interface {
	IsUpdateTask()
}
