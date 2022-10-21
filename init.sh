#!/usr/bin/env bash
TESTBENCH_PROJ_ROOT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# populate .env
echo "# This .env file is generated. Do not edit. See init.sh" > $TESTBENCH_PROJ_ROOT_DIR/.env
echo TESTBENCH_PROJ_ROOT_DIR=$TESTBENCH_PROJ_ROOT_DIR >> $TESTBENCH_PROJ_ROOT_DIR/.env

. init_vcpkg.sh
. init_aws.sh

# Export env variables.
export $(cat $TESTBENCH_PROJ_ROOT_DIR/.env | sed '/^#/d' | xargs)