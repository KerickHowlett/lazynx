package context

// TODO: This may need to be refactored to support unidirectional dependencies
// -     and better scalability.
type ContextTree struct {
	Global Context
	Status Context
	// Files                       *WorkingTreeContext
	// Menu                        *MenuContext
	// Tags                        *TagsContext
	// Suggestions                 *SuggestionsContext
	Normal          Context
	NormalSecondary Context
	// Confirmation                *ConfirmationContext
	CommandLog Context

	// display contexts
	AppStatus     Context
	Options       Context
	SearchPrefix  Context
	Search        Context
	Information   Context
	Limit         Context
	StatusSpacer1 Context
	StatusSpacer2 Context
}
