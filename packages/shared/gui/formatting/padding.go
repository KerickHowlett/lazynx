package formatting

import (
	"strings"

	"packages/shared/gui/color"

	"github.com/samber/lo"
)

// WithPadding pads a string as much as you want
func WithPadding(str string, padding int, alignment Alignment) string {
	uncoloredStr := color.Decolorize(str)
	width := StringWidth(uncoloredStr)
	if padding < width {
		return str
	}
	space := strings.Repeat(" ", padding-width)
	if alignment == AlignLeft {
		return str + space
	} else {
		return space + str
	}
}

func getPaddedDisplayStrings(stringArrays [][]string, columnConfigs []ColumnConfig) []string {
	result := make([]string, 0, len(stringArrays))
	for _, stringArray := range stringArrays {
		if len(stringArray) == 0 {
			continue
		}
		builder := strings.Builder{}
		for j, columnConfig := range columnConfigs {
			if len(stringArray)-1 < j {
				continue
			}
			builder.WriteString(WithPadding(stringArray[j], columnConfig.Width, columnConfig.Alignment))
			builder.WriteString(" ")
		}
		if len(stringArray)-1 < len(columnConfigs) {
			continue
		}
		builder.WriteString(stringArray[len(columnConfigs)])
		result = append(result, builder.String())
	}
	return result
}

func getPadWidths(stringArrays [][]string) []int {
	maxWidth := MaxFn(stringArrays, func(stringArray []string) int {
		return len(stringArray)
	})

	if maxWidth-1 < 0 {
		return []int{}
	}
	return lo.Map(lo.Range(maxWidth-1), func(i int, _ int) int {
		return MaxFn(stringArrays, func(stringArray []string) int {
			uncoloredStr := color.Decolorize(stringArray[i])

			return StringWidth(uncoloredStr)
		})
	})
}
