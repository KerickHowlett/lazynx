package text

type Sprinter interface {
	Sprint(a ...interface{}) string
	Sprintf(format string, a ...interface{}) string
}

func (b TextStyle) Sprint(a ...interface{}) string {
	return b.Style.Sprint(a...)
}

func (b TextStyle) Sprintf(format string, a ...interface{}) string {
	return b.Style.Sprintf(format, a...)
}
