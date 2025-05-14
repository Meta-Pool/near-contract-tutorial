if [ "$#" -ne 5 ]; then
    echo "Se requieren 5 parámetros: el nombre de la cuenta a crear, el owner inicial del contrato, el owner a editar, la cuenta del beneficiario cuando se elimine la cuenta y una cuenta para testear que no sea de owner"
    exit 1
fi

ACCOUNT_ID=$1
OWNER=$2
NEW_OWNER=$3
BENEFICIARY_ACCOUNT_ID=$4
NON_OWNER_ACCOUNT_ID=$5

CREATED_ACCOUNT_IDS=()

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

try_set_greeting_with_wrong_account() {
    echo "Creando una cuenta para testear que llamar al saludo falla"
    create_account $NON_OWNER_ACCOUNT_ID

    echo "Testeando set_greeting con una cuenta que no es owner..."
    set +e
    ./scripts/set_greeting.sh $ACCOUNT_ID "$NEW_OWNER" $NON_OWNER_ACCOUNT_ID
    set -e
    echo "Testando nuevamente el saludo..."
    ./scripts/test_greeting.sh $ACCOUNT_ID "$NEW_OWNER"
    echo "El saludo sigue siendo siendo el mismo"
}

try_deploy_and_test() {
    set -e
    echo "Deployando el contrato..."
    ./scripts/deploy.sh $ACCOUNT_ID "$OWNER"
    # echo "Testando el saludo..."
    # ./scripts/test_greeting.sh $ACCOUNT_ID "$OWNER"
    # echo "El saludo es correcto"
    # echo "Seteando el saludo..."
    # ./scripts/set_greeting.sh $ACCOUNT_ID "$NEW_OWNER" $ACCOUNT_ID
    # echo "Testando nuevamente el saludo..."
    # ./scripts/test_greeting.sh $ACCOUNT_ID "$NEW_OWNER"
    # echo "El saludo sigue siendo correcto"

    ./scripts/test.sh $ACCOUNT_ID "$OWNER" "$NEW_OWNER" $BENEFICIARY_ACCOUNT_ID $NON_OWNER_ACCOUNT_ID
    # test_types

    # ./scripts/log/log.sh $ACCOUNT_ID

    # try_set_greeting_with_wrong_account
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

remove_accounts() {
    ACCOUNT_ID_TO_REMOVE=$1
    echo "Eliminando las cuentas creadas..."
    if [[ ${#created_accounts[@]} -eq 0 ]]; then
        echo "No hay cuentas que eliminar"
        return
    fi
    for account_id in "${created_accounts[@]}"; do
        if [[ -n "$account_id" ]]; then
            echo "Eliminando la cuenta con ID: $account_id"
            ./scripts/remove_account.sh $account_id $BENEFICIARY_ACCOUNT_ID
            echo "Cuenta eliminada correctamente."
        fi
    done
    echo "Todas las cuentas han sido eliminadas."
}

create_account() {
    local account_id=$1
    echo "Creando cuenta con ID: $account_id"
    ./scripts/create_account.sh $account_id
    created_accounts+=("$account_id")
    echo "Cuenta creada correctamente."
}

echo "Corriendo con parámetros $ACCOUNT_ID, $OWNER, $NEW_OWNER y $BENEFICIARY_ACCOUNT_ID"

echo "Buildeando el contrato..."
./scripts/build.sh
echo "Creando la cuenta..."
create_account $ACCOUNT_ID

trap catch ERR
trap remove_accounts EXIT
try_deploy_and_test



