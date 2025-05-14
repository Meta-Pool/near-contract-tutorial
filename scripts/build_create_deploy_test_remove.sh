if [ "$#" -ne 4 ]; then
    echo "Se requieren 4 parámetros: el nombre de la cuenta a crear, el saludo inicial del contrato, el saludo a editar y la cuenta del beneficiario cuando se elimine la cuenta"
    exit 1
fi

ACCOUNT_ID=$1
GREETING=$2
NEW_GREETING=$3
BENEFICIARY_ACCOUNT_ID=$4

test_types() {
    echo "Testeando u8..."
    ./scripts/type_testing/u8.sh $ACCOUNT_ID
    echo "El u8 es correcto"

    echo "Testeando u128_badly..."
    ./scripts/type_testing/u128_badly.sh $ACCOUNT_ID
    echo "El u128_badly es correcto"

    echo "Testeando u128..."
    ./scripts/type_testing/U128.sh $ACCOUNT_ID
    echo "El U128 es correcto"

    echo "Testeando str_ref..."
    ./scripts/type_testing/str_ref.sh $ACCOUNT_ID
    echo "El str_ref es correcto"

    echo "Testeando bool..."
    ./scripts/type_testing/bool.sh $ACCOUNT_ID
    echo "El bool es correcto"

    echo "Testeando obj..."
    ./scripts/type_testing/obj.sh $ACCOUNT_ID
    echo "El obj es correcto"

    echo "Testeando vec..."
    ./scripts/type_testing/vec.sh $ACCOUNT_ID
    echo "El vec es correcto"

    echo "Testeando option_none..."
    ./scripts/type_testing/option_none.sh $ACCOUNT_ID
    echo "El option_none es correcto"

    echo "Testeando option_some..."
    ./scripts/type_testing/option_some.sh $ACCOUNT_ID
    echo "El option_some es correcto"
}

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

    test_types
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



