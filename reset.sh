#!/bin/bash

# Store the current branch
current_branch=$(git rev-parse --abbrev-ref HEAD)

# Get all branches
branches=$(git branch -r | grep -v '\->' | sed "s,\x1B\[[0-9;]*[a-zA-Z],,g" | sed 's/origin\///' | sed 's/^[[:space:]]*//')

# Function to rename and commit changes
rename_and_commit() {
    # Rename instances of zkp to zkp
    find . -type f \( -name "*.js" -o -name "*.c" -o -name "*.toml" -o -name "*.lua" -o -name "*.lock" -o -name "*.sh" -o -name "*.json" -o -name "*.h" \) -exec sed -i 's/zkp/zkp/g' {} +

    # Check if there are changes
    if [[ -n $(git status -s) ]]; then
        git add .
        git commit -m "Rename zkp to zkp"
    fi
}

# Iterate through all branches
for branch in $branches; do
    echo "Processing branch: $branch"
    git checkout $branch
    rename_and_commit
    git push origin $branch
done

# Return to the original branch
git checkout $current_branch

echo "All branches have been processed and pushed."