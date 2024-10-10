package rendering

type MainViewPairs struct {
	Normal         MainContextPair
	MergeConflicts MainContextPair
	Staging        MainContextPair
	PatchBuilding  MainContextPair
}

type ViewUpdateOpts struct {
	Title    string
	SubTitle string

	Task UpdateTask
}
