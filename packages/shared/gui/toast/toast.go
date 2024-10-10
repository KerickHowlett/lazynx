package toast

type ToastKind int

const (
	ToastKindStatus ToastKind = iota
	ToastKindError
)
