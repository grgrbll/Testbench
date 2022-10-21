#!/usr/bin/env bash
TESTBENCH_PROJ_ROOT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd $TESTBENCH_PROJ_ROOT_DIR

LOGS_DIR="$TESTBENCH_PROJ_ROOT_DIR/logs"
AWS_DIR="$TESTBENCH_PROJ_ROOT_DIR/aws"

# installing vcpkg
if [ ! -d "$LOGS_DIR" ]; then
    mkdir $LOGS_DIR
fi

# installing vcpkg
if [ ! -d "$AWS_DIR" ]; then
    curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" > $LOGS_DIR/aws_install
    unzip awscliv2.zip >> $LOGS_DIR/aws_install
    sudo ./aws/install >> $LOGS_DIR/aws_install
fi