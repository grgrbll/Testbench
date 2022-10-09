import os


def clean():
    os.system("rm -r ./build/*")


def prebuild():
    pwd = os.getcwd()
    os.chdir("./build")
    os.system(
        "cmake -S ../ -B . -DCMAKE_TOOLCHAIN_FILE=/Users/apple/vcpkg/vcpkg/scripts/buildsystems/vcpkg.cmake")
    os.chdir(pwd)


def build():
    pwd = os.getcwd()
    os.chdir("./build")
    os.system("cmake --build .")
    os.chdir(pwd)


def test():
    pwd = os.getcwd()
    os.chdir("./build/tests/")
    os.system("./main")
    os.chdir(pwd)
