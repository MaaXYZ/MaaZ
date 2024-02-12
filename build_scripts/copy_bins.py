import os
import sys
import shutil

def main():
    os.chdir(os.path.join(os.path.dirname(os.path.realpath(__file__)), ".."))

    mode = sys.argv[1]
    if mode not in ["debug", "release"]:
        print("Invalid mode: " + mode)
        exit(1)

    binsDir = os.path.join("deps", "maafw", "bin")
    agentDir = os.path.join("deps", "maafw", "share", "MaaAgentBinary")
    agentTargteDir = os.path.join("src-tauri", "MaaAgentBinary")
    targetDir = os.path.join("src-tauri", "target", mode)
    if not os.path.exists(targetDir):
        os.makedirs(targetDir)
    for root, dirs, files in os.walk(binsDir):
        for file in files:
            src = os.path.join(root, file)
            dst = os.path.join(targetDir, file)
            print("Copying " + src + " to " + dst)
            shutil.copy2(src, dst)
    if not os.path.exists(agentTargteDir):
        os.makedirs(agentTargteDir)
    for root, dirs, files in os.walk(agentDir):
        for file in files:
            src = os.path.join(root, file)
            dst = os.path.join(agentTargteDir, file)
            print("Copying " + src + " to " + dst)
            shutil.copy2(src, dst)

if __name__ == "__main__":
    main()