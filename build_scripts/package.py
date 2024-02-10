import os
import sys
import platform
import zipfile

def main():
    os.chdir(os.path.join(os.path.dirname(os.path.realpath(__file__)), ".."))
    mode = sys.argv[1]
    system = platform.system().lower()
    if mode not in ["debug", "release"]:
        print("Invalid mode: " + mode)
        exit(1)
    if system == "windows":
        binary_file = os.path.join("src-tauri", "target", mode, "maa-z.exe")
    else:
        binary_file = os.path.join("src-tauri", "target", mode, "maa-z")
    libsDir = os.path.join("deps", "maafw", "bin")
    agentBinary = os.path.join("deps", "maafw", "share", "MaaAgentBinary")
    resources = os.path.join("src-tauri","resources")

    # Create build directory
    build_dir = os.path.join("build", mode)
    if not os.path.exists(build_dir):
        os.makedirs(build_dir)

    # Create zip file
    zip_file = os.path.join(build_dir, "maa-z-" + mode + ".zip")
    if os.path.exists(zip_file):
        os.remove(zip_file)
    print("Creating zip file: " + zip_file)
    with zipfile.ZipFile(zip_file, 'w') as zip_ref:
        zip_ref.write(binary_file, os.path.basename(binary_file))
        for root, dirs, files in os.walk(libsDir):
            for file in files:
                zip_ref.write(os.path.join(root, file), os.path.join(file))
        # add agentbinary maintainin the directory structure with MaaAgentBinary as the root
        for root, dirs, files in os.walk(agentBinary):
            for file in files:
                zip_ref.write(os.path.join(root, file), os.path.join("MaaAgentBinary", os.path.relpath(root, agentBinary), file))
        for root, dirs, files in os.walk(resources):
            for file in files:
                zip_ref.write(os.path.join(root, file), os.path.join("resources", os.path.relpath(root, resources), file))
    print("Done")

if __name__ == "__main__":
    main()