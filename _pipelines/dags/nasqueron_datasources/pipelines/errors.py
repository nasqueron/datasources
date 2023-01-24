#   -------------------------------------------------------------
#   Nasqueron Datasources :: pipelines : errors
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#   Project:        Nasqueron
#   License:        BSD-2-Clause
#   -------------------------------------------------------------


class WorkflowException(Exception):
    def __init__(self, message):
        super(WorkflowException, self).__init__(message)


class CommandException(WorkflowException):
    def __init__(self, message, exit_code, stderr):
        consolidated_message = "{} (exit code {}): {}".format(
            message, exit_code, stderr
        )
        super(CommandException, self).__init__(consolidated_message)
