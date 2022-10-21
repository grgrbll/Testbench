#!/usr/bin/env bash
TESTBENCH_PROJ_ROOT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd $TESTBENCH_PROJ_ROOT_DIR

LOGS_DIR="$TESTBENCH_PROJ_ROOT_DIR/logs"
VCPKG_DIR="$TESTBENCH_PROJ_ROOT_DIR/vcpkg"

if [ ! -d "$LOGS_DIR" ]; then
    mkdir $LOGS_DIR
fi

# installing vcpkg
if [ ! -d "$VCPKG_DIR" ]; then
    git clone https://github.com/Microsoft/vcpkg.git
    ./vcpkg/bootstrap-vcpkg.sh
fi

export CMAKE_TOOLCHAIN_FILE="$VCPKG_DIR/scripts/buildsystems/vcpkg.cmake"

echo CMAKE_TOOLCHAIN_FILE=$CMAKE_TOOLCHAIN_FILE >> $TESTBENCH_PROJ_ROOT_DIR/.env

# Bad practice, but more reliable for codespaces
#export PATH=$PATH:"$VCPKG_DIR"
if [ ! -f /usr/local/bin/vcpkg ]
then
    sudo ln -s $VCPKG_DIR/vcpkg /usr/local/bin/vcpkg
fi

install_package() {
    if [ ! -f $LOGS_DIR/vcpkg_install_$1 ]
    then
        echo "Installing vpkg package $p..."
        vcpkg install $1 | tee $LOGS_DIR/vcpkg_install_$1
    fi
    echo "vpc package $p is installed."
}

while IFS="" read -r p || [ -n "$p" ]
do
    install_package "$p"
done < vcpkg_package_list.txt

