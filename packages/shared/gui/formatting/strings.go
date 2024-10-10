package formatting

import (
	"fmt"
	"slices"
	"strings"
	"unicode"

	"github.com/mattn/go-runewidth"
)

// Returns comma-separated list of paths, with ellipsis if there are more than 3
// e.g. "foo, bar, baz, [...3 more]"
func FormatPaths(paths []string) string {
	if len(paths) <= 3 {
		return strings.Join(paths, ", ")
	}
	return fmt.Sprintf("%s, %s, %s, [...%d more]", paths[0], paths[1], paths[2], len(paths)-3)
}

// defaults to left-aligning each column. If you want to set the alignment of
// each column, pass in a slice of Alignment values.
// returns a list of strings that should be joined with "\n", and an array of
// the column positions
func RenderDisplayStrings(displayStringsArr [][]string, columnAlignments []Alignment) ([]string, []int) {
	displayStringsArr, columnAlignments, removedColumns := excludeBlankColumns(displayStringsArr, columnAlignments)
	padWidths := getPadWidths(displayStringsArr)
	columnConfigs := make([]ColumnConfig, len(padWidths))
	columnPositions := make([]int, len(padWidths)+1)
	columnPositions[0] = 0
	for i, padWidth := range padWidths {
		// gracefully handle when columnAlignments is shorter than padWidths
		alignment := AlignLeft
		if len(columnAlignments) > i {
			alignment = columnAlignments[i]
		}

		columnConfigs[i] = ColumnConfig{
			Width:     padWidth,
			Alignment: alignment,
		}
		columnPositions[i+1] = columnPositions[i] + padWidth + 1
	}
	// Add the removed columns back into columnPositions (a removed column gets
	// the same position as the following column); clients should be able to rely
	// on them all to be there
	for _, removedColumn := range removedColumns {
		if removedColumn < len(columnPositions) {
			columnPositions = slices.Insert(columnPositions, removedColumn, columnPositions[removedColumn])
		}
	}
	return getPaddedDisplayStrings(displayStringsArr, columnConfigs), columnPositions
}

func SafeTruncate(str string, limit int) string {
	if len(str) > limit {
		return str[0:limit]
	}
	return str
}

func StringWidth(s string) int {
	// We are intentionally not using a range loop here, because that would
	// convert the characters to runes, which is unnecessary work in this case.
	for i := 0; i < len(s); i++ {
		if s[i] > unicode.MaxASCII {
			return runewidth.StringWidth(s)
		}
	}

	return len(s)
}

// TruncateWithEllipsis returns a string, truncated to a certain length, with an ellipsis
func TruncateWithEllipsis(str string, limit int) string {
	if StringWidth(str) > limit && limit <= 2 {
		return strings.Repeat(".", limit)
	}
	return runewidth.Truncate(str, limit, "…")
}
