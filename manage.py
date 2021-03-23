#!/usr/bin/env python3

import argparse
import json
import os
import subprocess

from typing import List


CYAN = "\033[96m {}\033[00m"
GREEN = "\033[92m {}\033[00m"
RED = "\033[91m {}\033[00m"

CONFIGURATION = json.load(open(".managerc"))


def add_environment_variable(environment: dict, key: str, value: str):
    if key in environment:
        environment[key] = "%s:%s" % (value, environment[key])
    else:
        environment[key] = value


def setup_environment():
    environment = os.environ.copy()
    HOME = environment['HOME']
    ELROND_SDK = "%s/elrondsdk" % HOME
    add_environment_variable(environment, "PATH", "%s/vendor-rust/bin" % ELROND_SDK)
    add_environment_variable(environment, "PATH", "%s/erdpy-venv/bin" % ELROND_SDK)
    add_environment_variable(environment, "PATH", "%s/arwentools" % ELROND_SDK)
    add_environment_variable(environment, "PATH", "%s/nodejs/latest/bin" % ELROND_SDK)
    add_environment_variable(environment, "VIRTUAL_ENV", "%s/erdpy-venv" % ELROND_SDK)
    add_environment_variable(environment, "RUSTUP_HOME", "%s/vendor-rust" % ELROND_SDK)
    add_environment_variable(environment, "CARGO_HOME", "%s/vendor-rust" % ELROND_SDK)
    return environment


def execute_command(command: str):
    environment = setup_environment()
    print(CYAN.format("> %s" % command))
    process = subprocess.Popen(command, shell=True, env=environment, stdout=subprocess.PIPE)
    process.wait()
    out, err = process.communicate()
    print(out.decode())
    if b'ERROR' not in out:
        print(GREEN.format("OK"))
    else:
        print(RED.format("KO"))
        exit(0)


def install():
    INSTALL_COMMAND = "erdpy deps install rust --overwrite --tag %s" 
    execute_command(INSTALL_COMMAND % CONFIGURATION['rust_version'])


def clean(contracts: List[str]):
    CLEAN_COMMAND = "erdpy --verbose contract clean %s"
    for contract in contracts:
        execute_command(CLEAN_COMMAND % contract)


def build(contracts: List[str]):
    BUILD_COMMAND = "cd %s && erdpy --verbose contract build"
    for contract in contracts:
        execute_command(BUILD_COMMAND % (contract))


def test(contracts: List[str]):
    TEST_COMMAND = "cd %s && mandos-test ."
    for contract in contracts:
        execute_command(TEST_COMMAND % contract)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--install', dest='install', action='store_true', default=False)
    parser.add_argument('--clean', dest='clean', action='store_true', default=False)
    parser.add_argument('--build', dest='build', action='store_true', default=False)
    parser.add_argument('--test', dest='test', action='store_true', default=False)
    parser.add_argument('--contracts', dest='contracts', nargs='+', default=CONFIGURATION['contracts'])
    args = parser.parse_args()
    setup_environment()
    if args.install:
        install()
    elif args.clean:
        clean(args.contracts)
    else:
        if args.build:
            build(args.contracts)
        if args.test:
            test(args.contracts)


if __name__ == "__main__":
    main()

