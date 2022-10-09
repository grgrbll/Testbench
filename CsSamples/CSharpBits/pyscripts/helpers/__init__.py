import os


def clean():
    pass
    #os.system("rm -r ./build/*")


def prebuild():
    pass
#    pwd = os.getcwd()
#    os.chdir("./build")
#    os.system(
#        "cmake -S ../ -B . -DCMAKE_TOOLCHAIN_FILE=/Users/apple/vcpkg/vcpkg/scripts/buildsystems/vcpkg.cmake")
#    os.chdir(pwd)


def build():
#   pwd = os.getcwd()
#   os.chdir("./build")
    os.system("dotnet build")
#   os.chdir(pwd)


def test():
#   pwd = os.getcwd()
#   os.chdir("./build/tests/")
    os.system("dotnet test")
#   os.chdir(pwd)
