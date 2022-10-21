import shutil
from dotenv import load_dotenv

def load_env():
    load_dotenv()

def check_env():
    success_count = 0
    fail_count = 0
    def _check_tool(toolname):
        nonlocal success_count
        nonlocal fail_count
        toolpath = shutil.which(toolname)
        if toolpath is not None:
            print(f"[x] tool \"{toolname}\" found at: {toolpath}")
            success_count += 1
        else:
            print(f"[ ] tool \"{toolname}\" not found.")
            fail_count += 1

    _check_tool("aws")
    _check_tool("vcpkg")

    print(f"Tools: {success_count}/{fail_count + success_count}")
    print(";".join( open(".env", "r").readlines()))