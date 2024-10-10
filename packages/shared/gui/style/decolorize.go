package style

import (
	"regexp"
	"sync"
)

var (
	decolorizeCache = make(map[string]string)
	decolorizeMutex sync.RWMutex
)

// Decolorize strips a string of color
func Decolorize(str string) string {
	decolorizeMutex.RLock()
	val := decolorizeCache[str]
	decolorizeMutex.RUnlock()

	if val != "" {
		return val
	}

	re := regexp.MustCompile(`\x1B\[([0-9]{1,3}(;[0-9]{1,3})*)?[mGK]`)
	ret := re.ReplaceAllString(str, "")

	decolorizeMutex.Lock()
	decolorizeCache[str] = ret
	decolorizeMutex.Unlock()

	return ret
}
