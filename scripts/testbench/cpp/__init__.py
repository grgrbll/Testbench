import os
import shutil

def list_projects():
    result = []
    rootdir = os.environ["TESTBENCH_PROJ_ROOT_DIR"]
    for p in os.listdir(rootdir):
        if os.path.isdir(p) and "CMakeLists.txt" in os.listdir(p) :
            result.append(os.path.join(rootdir, p))
    return result

def clean_all():
    for proj in list_projects():
        try:
            clean(proj)
        except Exception as e:
            print(f"Failed to clean {proj}: " + str(e))

def prebuild_all():
    for proj in list_projects():
        prebuild(proj)

def build_all():
    for proj in list_projects():
        build(proj)

def prebuild(path):
    toolchain_file_path = os.environ["CMAKE_TOOLCHAIN_FILE"]
    build_dir = os.path.join(path, "build")
    pwd = os.getcwd()
    if not os.path.exists(build_dir):
        os.mkdir(build_dir)
    os.chdir(build_dir)
    cmake_cmd = f"cmake -S ../ -B . -DCMAKE_TOOLCHAIN_FILE={toolchain_file_path}"
    print(cmake_cmd)
    os.system(cmake_cmd)
    os.chdir(pwd)

def build(path):
    build_dir = os.path.join(path, "build")
    pwd = os.getcwd()
    os.chdir(build_dir)
    os.system("cmake --build .")
    os.chdir(pwd)

def clean(path):
    build_dir = os.path.join(path, "build")

    if not os.path.isdir(build_dir):
        raise Exception("Expected CMakeFiles in build dir.")

    if "CMakeFiles" not in os.listdir(build_dir):
        raise Exception("Expected CMakeFiles in build dir.")

    shutil.rmtree(build_dir)