# !/bin/bash

# This script enters a bash shell in the jolpica-f1 devcontainer, which was likely started via
# `setup_and_run_local_jolpica_server.sh`, at the /workspaces/f1_data directory, and with the
# LOCAL_JOLPICA env variable set to 1, ready for testing against a local jolpica-f1 server.

container_id="dc8a9c4dd4f227db60c2a9c8e5a3abff65ff9cdc81d90f4cfe3129cd361ccadd"

devcontainer exec --container-id $container_id \
    bash -c "cd /workspaces/f1_data; export LOCAL_JOLPICA=1; bash"
