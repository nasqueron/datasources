#   -------------------------------------------------------------
#   Nasqueron Datasources :: pipelines :: command utilities
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#   Project:        Nasqueron
#   Description:    Helpers to handle commands in Python pipelines
#   License:        BSD-2-Clause
#   -------------------------------------------------------------


import subprocess


#   -------------------------------------------------------------
#   Subprocess wrappers
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


def run(command, cwd=None, env=None):
    """
    Runs the specified command and return exit_code, stdout, stderr.

    :type env: dict|None
    :param env: The environment variables to pass to the software
    :type command: string|list
    :param command: The command to run, as a string to pass to shell (to avoid) or a list [command, arg1, arg2, ...]
    :param cwd: The working directory for the command to run

    :return: (exit_code, stdout, stderr)
    """
    if env is None:
        env = {}
    shell = type(command) is str
    process = subprocess.run(
        command, shell=shell, cwd=cwd, env=env, capture_output=True
    )

    return process.returncode, process.stdout, process.stderr


#   -------------------------------------------------------------
#   Environment
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -


def parse_environment(environment_lines):
    """
    Parses environment as a dictionary.

    This method is intended to be used with `env`, with .env files,
    or with any command offering a similar format:

    VARIABLE=value
    """
    return {
        parts[0]: parts[1]
        for parts in [line.strip().split("=") for line in environment_lines]
    }
