package utils

type StringStack struct {
	stack []string
}

func (ss *StringStack) Push(s string) {
	ss.stack = append(ss.stack, s)
}

func (ss *StringStack) Pop() string {
	if len(ss.stack) == 0 {
		return ""
	}
	n := len(ss.stack) - 1
	last := ss.stack[n]
	ss.stack = ss.stack[:n]
	return last
}

func (ss *StringStack) IsEmpty() bool {
	return len(ss.stack) == 0
}

func (ss *StringStack) Clear() {
	ss.stack = []string{}
}
