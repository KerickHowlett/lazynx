package rendering

import "os/exec"

type RunCommandTask struct {
	Cmd    *exec.Cmd
	Prefix string
}

func (t *RunCommandTask) IsUpdateTask() {}

func NewRunCommandTask(cmd *exec.Cmd) *RunCommandTask {
	return &RunCommandTask{Cmd: cmd}
}

func NewRunCommandTaskWithPrefix(cmd *exec.Cmd, prefix string) *RunCommandTask {
	return &RunCommandTask{Cmd: cmd, Prefix: prefix}
}
