import os
import subprocess

for path in map(lambda path: path.replace(".rs", ""), filter(lambda path: path.endswith(".rs"), os.listdir("examples"))):
    subprocess.run(["cargo", "run", "--example", path])