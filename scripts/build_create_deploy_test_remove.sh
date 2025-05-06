if [ "$#" -ne 4 ]; then
    echo "Se requieren 4 parámetros: el nombre de la cuenta a crear, el saludo inicial del contrato, el saludo a editar y la cuenta del beneficiario cuando se elimine la cuenta"
    exit 1
fi

ACCOUNT_ID=$1
GREETING=$2
NEW_GREETING=$3
BENEFICIARY_ACCOUNT_ID=$4

try_deploy_and_test() {
    set -e
    echo "Deployando el contrato..."
    ./scripts/deploy.sh $ACCOUNT_ID "$GREETING"
    echo "Testando el saludo..."
    ./scripts/test_greeting.sh $ACCOUNT_ID "$GREETING"
    echo "El saludo es correcto"
    echo "Seteando el saludo..."
    ./scripts/set_greeting.sh $ACCOUNT_ID "$NEW_GREETING"
    echo "Testando nuevamente el saludo..."
    ./scripts/test_greeting.sh $ACCOUNT_ID "$NEW_GREETING"
    echo "El saludo sigue siendo correcto"
}

catch() {
    local exit_code=$1
    if [ $exit_code -ne 0 ]; then
        echo "Deployment failed with exit code $exit_code"
        return $exit_code
    fi
    echo "Deployment successful"
    return 0
}

remove_account() {
    echo "Eliminando la cuenta..."
    ./scripts/remove_account.sh $ACCOUNT_ID $BENEFICIARY_ACCOUNT_ID
    echo "Cuenta eliminada correctamente."
}

echo "Corriendo con parámetros $ACCOUNT_ID, $GREETING, $NEW_GREETING y $BENEFICIARY_ACCOUNT_ID"

echo "Buildeando el contrato..."
./scripts/build.sh
echo "Creando la cuenta..."
./scripts/create_account.sh $ACCOUNT_ID

trap catch ERR
trap remove_account EXIT
try_deploy_and_test



