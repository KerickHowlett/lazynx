package popup

import (
	"packages/shared/gui/menu"
	"packages/shared/gui/suggestion"
	"packages/shared/gui/toast"

	"github.com/jesseduffield/gocui"
)

type IPopupHandler interface {
	// The global error handler for gocui. Not to be used by application code.
	ErrorHandler(err error) error
	// Shows a notification popup with the given title and message to the user.
	//
	// This is a convenience wrapper around Confirm(), thus the popup can be closed using both 'Enter' and 'ESC'.
	Alert(title string, message string) error
	// Shows a popup asking the user for confirmation.
	Confirm(opts ConfirmOpts) error
	// Shows a popup prompting the user for input.
	Prompt(opts PromptOpts) error
	WithWaitingStatus(message string, f func(gocui.Task) error) error
	WithWaitingStatusSync(message string, f func() error) error
	Menu(opts menu.CreateMenuOptions) error
	Toast(message string)
	ErrorToast(message string)
	SetToastFunc(func(string, toast.ToastKind))
	GetPromptInput() string
}

type ConfirmOpts struct {
	Title               string
	Prompt              string
	HandleConfirm       func() error
	HandleClose         func() error
	FindSuggestionsFunc func(string) []*suggestion.Suggestion
	Editable            bool
	Mask                bool
}

type PromptOpts struct {
	Title               string
	InitialContent      string
	FindSuggestionsFunc func(string) []*suggestion.Suggestion
	HandleConfirm       func(string) error
	AllowEditSuggestion bool

	// CAPTURE THIS
	HandleClose            func() error
	HandleDeleteSuggestion func(int) error
	Mask                   bool
}
