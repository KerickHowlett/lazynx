package formatting

import "slices"

type ColumnConfig struct {
	Width     int
	Alignment Alignment
}

// NOTE: this mutates the input slice for the sake of performance
func excludeBlankColumns(displayStringsArr [][]string, columnAlignments []Alignment) ([][]string, []Alignment, []int) {
	if len(displayStringsArr) == 0 {
		return displayStringsArr, columnAlignments, []int{}
	}

	// if all rows share a blank column, we want to remove that column
	toRemove := []int{}
outer:
	for i := range displayStringsArr[0] {
		for _, strings := range displayStringsArr {
			if strings[i] != "" {
				continue outer
			}
		}
		toRemove = append(toRemove, i)
	}

	if len(toRemove) == 0 {
		return displayStringsArr, columnAlignments, []int{}
	}

	// remove the columns
	for i, strings := range displayStringsArr {
		for j := len(toRemove) - 1; j >= 0; j-- {
			strings = slices.Delete(strings, toRemove[j], toRemove[j]+1)
		}
		displayStringsArr[i] = strings
	}

	for j := len(toRemove) - 1; j >= 0; j-- {
		if columnAlignments != nil && toRemove[j] < len(columnAlignments) {
			columnAlignments = slices.Delete(columnAlignments, toRemove[j], toRemove[j]+1)
		}
	}

	return displayStringsArr, columnAlignments, toRemove
}
