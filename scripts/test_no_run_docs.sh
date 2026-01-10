# !/bin/bash

# This script tests all documentation tests marked with ```no_run. It temporarily removes the
# ```no_run markers, applies a patch to add rate limiting to the jolpica::get::get_response_page
# function or, if --local_jolpica is passed, to change jolpica::api::JOLPICA_API_BASE_URL to
# "http://localhost:8000/ergast/f1" and disable default rate limiting. It then runs the
# documentation tests, and lastly reverts all changes. Since it makes multiple code modifications
# that need to be reverted, it first checks that the working directory is clean before proceeding,
# in order to avoid losing any uncommitted changes.

local_jolpica=false

for i in "$@"; do
  case $i in
    --local_jolpica)
      local_jolpica=true
      shift
      ;;
    -*|--*)
      echo "Unknown option $i"
      exit 1
      ;;
    *)
      ;;
  esac
done

if [[ -n "$(git status -s)" ]]; then
     echo "There are uncommitted changes in the working directory."
     echo "Please commit or stash them before running this script."
     exit 1
fi

# Remove all occurrences of ```no_run in .rs files
find ./src -type f -name "*.rs" -exec sed -i 's/```no_run/```/g' {} +

if [ "$local_jolpica" == true ]; then
    # Apply patch to change JOLPICA_API_BASE_URL and disable rate limiting
    git apply ./patches/no_run_doc_tests_local_jolpica.patch
else
     # Apply rate limiting patch to ./src/jolpica/get.rs
     git apply ./patches/no_run_doc_tests_add_rate_limiting.patch
fi

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
