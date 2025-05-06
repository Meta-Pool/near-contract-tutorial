if [ "$#" -ne 2 ]; then
    echo "Deployar el contrato requiere dos parámetros: el nombre de la cuenta y el saludo inicial del contrato. Argumentos: $#: $@"
    exit 1
fi

ACCOUNT_ID=$1
GREETING=$2

near deploy $ACCOUNT_ID ./target/wasm32-unknown-unknown/release/near_contract_tutorial.wasm --initFunction new --initArgs '{"initial_greeting": "'"$GREETING"'"}'