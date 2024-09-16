package context

import (
	"packages/shared/gui/keybindings"

	"github.com/jesseduffield/gocui"
)

type IBaseContext interface {
	keybindings.HasKeybindings
	// ParentContext

	// GetKind() ContextKind
	GetViewName() string
	GetView() *gocui.View
	// GetViewTrait() IViewTrait
	GetWindowName() string
	SetWindowName(string)
	GetKey() keybindings.ContextKey
	CanBeFocused() bool
	// if a context is transient, then it only appears via some keybinding on another
	// context. Until we add support for having multiple of the same context, no two
	// of the same transient context can appear at once meaning one might be 'stolen'
	// from another window.
	IsTransient() bool
	// this tells us if the view's bounds are determined by its window or if they're
	// determined independently.
	HasControlledBounds() bool

	// to what extent the view needs to be rerendered when its width changes
	// NeedsRerenderOnWidthChange() NeedsRerenderOnWidthChangeLevel

	// true if the view needs to be rerendered when its height changes
	NeedsRerenderOnHeightChange() bool

	// returns the desired title for the view upon activation. If there is no desired title (returns empty string), then
	// no title will be set
	Title() string

	GetOptionsMap() map[string]string

	AddKeybindingsFn(keybindings.KeybindingsFn)
	AddMouseKeybindingsFn(keybindings.MouseKeybindingsFn)
	ClearAllBindingsFn()

	// This is a bit of a hack at the moment: we currently only set an onclick function so that
	// our list controller can come along and wrap it in a list-specific click handler.
	// We'll need to think of a better way to do this.
	AddOnClickFn(func() error)

	AddOnRenderToMainFn(func() error)
	AddOnFocusFn(func(keybindings.OnFocusOpts) error)
	AddOnFocusLostFn(func(keybindings.OnBlurOpts) error)
}
