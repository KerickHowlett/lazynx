package rendering

import "os/exec"

type RunPtyTask struct {
	Cmd    *exec.Cmd
	Prefix string
}

func (t *RunPtyTask) IsUpdateTask() {}

func NewRunPtyTask(cmd *exec.Cmd) *RunPtyTask {
	return &RunPtyTask{Cmd: cmd}
}
