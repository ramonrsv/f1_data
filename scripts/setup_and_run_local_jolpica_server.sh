# !/bin/bash

# This script sets up and runs a local instance of the jolpica-f1 server for testing purposes. It
# requires a destination directory as an argument, where it will clone the jolpica-f1 repository.
# It will `npm install @devcontainers/cli`, start the .devcontainer environment, download the
# latest database dump, setup the database, and run the server at "http://localhost:8000/ergast/f1".

container_id="dc8a9c4dd4f227db60c2a9c8e5a3abff65ff9cdc81d90f4cfe3129cd361ccadd"
db_csv_file_name="jolpica-f1-csv.zip"

dst=$1

if [ -z "$dst" ]; then
    echo "Usage: $0 <destination_directory>"
    exit 1
fi

if [ ! -d "$dst" ]; then
    echo "Creating destination directory $dst"
    mkdir -p "$dst"
fi

echo "Cloning jolpica-f1 repository into $dst/jolpica-f1"
git clone git@github.com:jolpica/jolpica-f1.git "$dst/jolpica-f1" || exit 1

echo "Applying patch to disable rate limiting"
cp -avr ./scripts/disable_jolpica_server_rate_limits.patch "$dst/jolpica-f1/"
cd "$dst/jolpica-f1"
git apply "./disable_jolpica_server_rate_limits.patch" || exit 1

echo "Downloading the latest database dump"
wget "https://api.jolpi.ca/data/dumps/download/delayed/?dump_type=csv" -O "$db_csv_file_name" || exit 1

echo "Installing @devcontainers/cli"
npm install @devcontainers/cli || exit 1

echo "Starting the .devcontainer environment"
devcontainer up --workspace-folder . || exit 1

echo "Continuing inside the container, ID: $container_id"

function devcontainer_exec() {
    args="$@"
    devcontainer exec --container-id $container_id bash -c "cd /workspaces/jolpica-f1; $args"
}

echo "Setting up the database and running tests"

devcontainer_exec make setup
devcontainer_exec ./scripts/restore_from_csv_dump.sh localhost jolpica postgres
devcontainer_exec pytest --create-db

echo "Running the jolpica-f1 server at http://localhost:8000/ergast/f1"
echo "Enter container: devcontainer exec --container-id $container_id bash"
echo "----------------------------------"

devcontainer_exec make run
