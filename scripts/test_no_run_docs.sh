# !/bin/bash

# This script tests all documentation tests marked with ```no_run. It temporarily removes the
# ```no_run markers, applies a patch to add rate limiting to the jolpica::get::get_response_page
# function, runs the documentation tests, and then reverts all changes. Since it may be run
# locally with 'act', it first checks that the working directory is clean before proceeding, in
# order to avoid losing any uncommitted changes.

if [[ -n "$(git status -s)" ]]; then
     echo "There are uncommitted changes in the working directory."
     echo "Please commit or stash them before running this script."
     exit 1
fi

# Remove all occurrences of ```no_run in .rs files
find ./src -type f -name "*.rs" -exec sed -i 's/```no_run/```/g' {} +

# Apply rate limiting patch to ./src/jolpica/get.rs
git apply ./scripts/get_response_page_rate_limiting.patch

# Log the applied changes for verification
# @todo This causes `act` to freeze, for some reason
# git diff

# Run ```no_run documentation tests
cargo test --doc -- --test-threads=1
test_exit_code=$?

# Revert changes made to the working directory
git reset HEAD --hard

# Exit with the test exit code
exit $test_exit_code
