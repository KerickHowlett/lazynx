package utils

import (
	"io"
	"sync"
)

// This wraps a writer and ensures that before we actually write anything we call a given function first

type OnceWriter struct {
	writer io.Writer
	once   sync.Once
	f      func()
}

var _ io.Writer = &OnceWriter{}

func NewOnceWriter(writer io.Writer, f func()) *OnceWriter {
	return &OnceWriter{
		writer: writer,
		f:      f,
	}
}

func (ow *OnceWriter) Write(p []byte) (n int, err error) {
	ow.once.Do(func() {
		ow.f()
	})

	return ow.writer.Write(p)
}
