package app

type App struct{}

func NewApp() (app *App, err error) {
	app = &App{}

	return app, nil
}

func Run() {
	app, err := NewApp()

	if err != nil {
		panic(err)
	}

	app.Run()
}

func (app *App) Run() error {
	return nil
}

func (app *App) Close() error {
	return nil
}
