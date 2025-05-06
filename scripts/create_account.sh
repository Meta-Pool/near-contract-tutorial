if [ "$#" -ne 1 ]; then
    echo "Crear una cuenta de Near solo acepta 1 parámetro"
    exit 1
fi

ACCOUNT_ID=$1

near create-account $ACCOUNT_ID --useFaucet