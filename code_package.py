import os
import re
from termcolor import colored

# Define the root directory of your project
root_dir = '.'
output_file = 'output.txt'

# Initialize an empty list to store the lines of code with directory information
lines = []

# List of encodings to try
encodings = ['utf-8', 'latin1', 'ascii']

# Load .gitignore patterns
gitignore_file = os.path.join(root_dir, '.gitignore')
ignore_patterns = []

if os.path.exists(gitignore_file):
    with open(gitignore_file, 'r', encoding='utf-8') as f:
        ignore_patterns = [line.strip() for line in f if line.strip() and not line.startswith('#')]

def is_ignored(path, ignore_patterns):
    """Check if the given path matches any of the ignore patterns."""
    for pattern in ignore_patterns:
        if re.search(re.escape(pattern).replace('\\*', '.*'), path):
            return True
    return False

# Walk through the directory tree
for dirpath, dirnames, filenames in os.walk(root_dir):
    # Ignore the node_modules and .git directories
    for ignore_dir in ['node_modules', '.git']:
        if ignore_dir in dirnames:
            dirnames.remove(ignore_dir)

    for filename in filenames:
        # Construct the full file path
        file_path = os.path.join(dirpath, filename)

        # Check if the file path should be ignored
        ignore_patterns = ignore_patterns + ['output.txt', 'code_package.py', 'assets', '.gitignore', 'go.sum', '*.exe', 'tiledgame.*']
        if is_ignored(file_path, ignore_patterns):
            # add assets to ignore_patterns
            print(colored(f"🚫 Ignoring {file_path}", 'yellow'))
            continue

        content = None
        # Try reading the file with different encodings
        for encoding in encodings:
            try:
                with open(file_path, 'r', encoding=encoding) as file:
                    content = file.read()
                break
            except UnicodeDecodeError:
                continue

        # If the file could not be read with any encoding, skip it
        if content is None:
            print(colored(f"⚠️ Skipping {file_path} due to encoding issues", 'red'))
            continue

        # Add the directory and file information along with the content to the list
        lines.append(f"\n# {os.path.relpath(file_path, root_dir)}\n")
        lines.append(content)
        print(colored(f"📄 Added {file_path}", 'green'))

# Write the concatenated content to the output file
with open(output_file, 'w', encoding='utf-8') as output:
    output.writelines(lines)

print(colored(f"✅ Source code has been concatenated into {output_file}", 'cyan'))
