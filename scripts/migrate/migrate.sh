if [ "$#" -ne 2 ]; then
    echo "Redeployar el contrato requiere dos parámetros: el nombre de la cuenta del contrato y el owner del contrato. Argumentos: $#: $@"
    exit 1
fi

CONTRACT_ID=$1
OWNER=$2

near deploy $CONTRACT_ID ./target/wasm32-unknown-unknown/release/near_contract_tutorial.wasm --initFunction migrate --initArgs '{"new_owner": "'"$OWNER"'"}'