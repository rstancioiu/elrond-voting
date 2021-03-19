#!/usr/bin/env python3

import argparse
import os
import subprocess


CYAN = "\033[96m {}\033[00m"
GREEN = "\033[92m {}\033[00m"
RED = "\033[91m {}\033[00m"


def add_environment_variable(environment, key, value):
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


def execute_command(command):
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
    RUST_VERSION = "nightly-2021-02-26"
    INSTALL_COMMAND = "erdpy deps install rust --overwrite --tag %s" 
    execute_command(INSTALL_COMMAND % RUST_VERSION)


def clean():
    CLEAN_COMMAND = "erdpy --verbose contract clean \"%s\""
    execute_command(CLEAN_COMMAND % os.getcwd())


def build():
    BUILD_COMMAND = "erdpy --verbose contract build \"%s\""
    execute_command(BUILD_COMMAND % os.getcwd())


def test():
    TEST_COMMAND = "mandos-test \"%s\""
    execute_command(TEST_COMMAND % os.getcwd())


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--install', dest='install', action='store_true', default=False)
    parser.add_argument('--clean', dest='clean', action='store_true', default=False)
    parser.add_argument('--build', dest='build', action='store_true', default=True)
    parser.add_argument('--test', dest='test', action='store_true', default=True)
    args = parser.parse_args()
    setup_environment()
    if args.install:
        install()
    elif args.clean:
        clean()
    else:
        if args.build:
            build()
        if args.test:
            test()


if __name__ == "__main__":
    main()

