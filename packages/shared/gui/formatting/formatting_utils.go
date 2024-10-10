package formatting

func MaxFn[T any](items []T, fn func(T) int) int {
	max := 0
	for _, item := range items {
		if fn(item) > max {
			max = fn(item)
		}
	}
	return max
}
