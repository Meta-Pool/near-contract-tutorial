if [ "$#" -ne 5 ]; then
    echo "Se requieren 5 parámetros: el nombre del contrato a testear, el owner actual del contrato, el owner a editar, la cuenta del beneficiario cuando se elimine la cuenta y una cuenta para testear que no sea de owner"
    exit 1
fi

CONTRACT_ID=$1
OWNER=$2
NEW_OWNER=$3
BENEFICIARY_ACCOUNT_ID=$4
NON_OWNER_ACCOUNT_ID=$5

try_migrate_and_test() {
    echo "Migrando el contrato..."
    ./scripts/migrate/migrate.sh $CONTRACT_ID $OWNER
    echo "El contrato fue migrado correctamente."

    echo "Testeando el contrato..."
    ./scripts/test.sh $CONTRACT_ID $OWNER $NEW_OWNER $NON_OWNER_ACCOUNT_ID
    echo "El contrato fue testeado correctamente."
}

catch() {
    local exit_code=$1
    if [ $exit_code -ne 0 ]; then
        echo "Deployment failed with exit code $exit_code"
        return $exit_code
    else 
        echo "Deployment succeeded"
        return 0
    fi
}

remove_account() {
    echo "Eliminando la cuenta con ID: $CONTRACT_ID"
    ./scripts/remove_account.sh $CONTRACT_ID $BENEFICIARY_ACCOUNT_ID
    echo "Cuenta eliminada correctamente."
}

echo "Corriendo con parámetros $CONTRACT_ID, $OWNER, $NEW_OWNER, $BENEFICIARY_ACCOUNT_ID y $NON_OWNER_ACCOUNT_ID"

echo "Buildeando el contrato..."
./scripts/build.sh

trap catch ERR
trap remove_account EXIT
try_migrate_and_test