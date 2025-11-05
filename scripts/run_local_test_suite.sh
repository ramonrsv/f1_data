# !/bin/bash

# This script runs a local test suite roughly equivalent to the GitHub Actions CI workflow.
# It is meant to allow running the CI workflow against a local jolpica-f1 API server instance
# (see `setup_and_run_local_jolpica_server.sh` and `enter_jolpica_devcontainer_shell.sh`).
#
# @todo This should only be a temporary solution until support is added to the CI workflow (ci.yml)
# to setup and run a local jolpica-f1 server, then we could just use `act` to run the suite locally.

function print_usage() {
    echo "Usage: $0 [options]"
    echo "Options:"
    echo "  --test_no_run_docs  Run \`test_no_run_docs.sh\`; requires a clean repo (default: false)"
    echo "  --run_benchmarks    Run \`cargo bench ...\` to run benchmarks (default: false)"
    echo "  -h, --help          Show this help message"
}

test_no_run_docs=false
fun_benchmarks=false

for i in "$@"; do
  case $i in
    --test_no_run_docs)
      test_no_run_docs=true
      shift
      ;;
    --run_benchmarks)
      run_benchmarks=true
      shift
      ;;
    -h|--help)
      print_usage
      exit 0
      ;;
    -*|--*)
      echo "Unknown option $i"
      exit 1
      ;;
    *)
      ;;
  esac
done

# Executes a command and exits if the command fails
# $1: Name of the command (for error message)
# "${@:2}": Command and its arguments
function exec_exit_on_error() {
    local name="$1"
    local cmd="${@:2}"

    echo "➡️ $name: $cmd"
    $cmd
    local status=$?

    if [ $status -ne 0 ]; then
        echo "❌ ERROR: '$name' failed with exit code $status"
        exit $status
    else
        echo "✅ $name"
    fi
}

exec_exit_on_error "rustfmt"               cargo fmt --all --check
exec_exit_on_error "clippy"                cargo clippy --workspace --all-features --no-deps
exec_exit_on_error "Build docs"            cargo doc --workspace --all-features --no-deps --document-private-items
exec_exit_on_error "Build docs (examples)" cargo doc --examples --no-deps
exec_exit_on_error "Build crate"           cargo build --workspace --all-features
exec_exit_on_error "Test (non-ignored)"    cargo test --workspace --all-features
exec_exit_on_error "Test (ignored)"        cargo test --workspace --all-features -- --ignored --test-threads 1

if [ "$test_no_run_docs" = true ]; then
    exec_exit_on_error "\`\`\`no_run doc tests" ./scripts/test_no_run_docs.sh --local_jolpica
fi

if [ "$run_benchmarks" = true ]; then
    exec_exit_on_error "Benchmarks"        cargo bench --workspace --all-features
fi
