if [ "$#" -ne 5 ]; then
    echo "Se requieren 5 parámetros: el nombre del contrato a testear, el owner actual esperado, el nuevo owner a poner, el beneficiario para eliminar una cuenta y una cuenta para testear que no sea de owner"
    exit 1
fi

CONTRACT_ID=$1
OWNER=$2
NEW_OWNER=$3
BENEFICIARY_ACCOUNT_ID=$4
NON_OWNER_ACCOUNT_ID=$5

test_types() {
    echo "Testeando u8..."
    ./scripts/type_testing/u8.sh $CONTRACT_ID
    echo "El u8 es correcto"

    echo "Testeando u128_badly..."
    ./scripts/type_testing/u128_badly.sh $CONTRACT_ID
    echo "El u128_badly es correcto"

    echo "Testeando u128..."
    ./scripts/type_testing/U128.sh $CONTRACT_ID
    echo "El U128 es correcto"

    # echo "Testeando str_ref..."
    # ./scripts/type_testing/str_ref.sh $CONTRACT_ID
    # echo "El str_ref es correcto"

    echo "Testeando bool..."
    ./scripts/type_testing/bool.sh $CONTRACT_ID
    echo "El bool es correcto"

    echo "Testeando obj..."
    ./scripts/type_testing/obj.sh $CONTRACT_ID
    echo "El obj es correcto"

    echo "Testeando vec..."
    ./scripts/type_testing/vec.sh $CONTRACT_ID
    echo "El vec es correcto"

    echo "Testeando option_none..."
    ./scripts/type_testing/option_none.sh $CONTRACT_ID
    echo "El option_none es correcto"

    echo "Testeando option_some..."
    ./scripts/type_testing/option_some.sh $CONTRACT_ID
    echo "El option_some es correcto"
}

try_set_greeting_with_wrong_account() {
    echo "Creando una cuenta para testear que llamar al saludo falla"
    create_account $NON_OWNER_ACCOUNT_ID
    trap remove_accounts EXIT

    echo "Testeando set_owner con una cuenta que no es owner..."
    set +e
    ./scripts/set_owner.sh $CONTRACT_ID "$NEW_OWNER" $NON_OWNER_ACCOUNT_ID
    set -e
    echo "Testando nuevamente el owner..."
    ./scripts/test_owner.sh $CONTRACT_ID "$NEW_OWNER"
    echo "El owner sigue siendo siendo el mismo"
}

remove_account() {
    echo "Eliminando la cuenta con ID: $NON_OWNER_ACCOUNT_ID"
    ./scripts/remove_account.sh $NON_OWNER_ACCOUNT_ID $BENEFICIARY_ACCOUNT_ID
    echo "Cuenta eliminada correctamente."
}

remove_accounts() {
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

echo "Testando el owner..."
./scripts/test_owner.sh $CONTRACT_ID "$OWNER"
echo "El owner es correcto"
echo "Seteando el owner..."
./scripts/set_owner.sh $CONTRACT_ID "$NEW_OWNER" $CONTRACT_ID
echo "Testando nuevamente el owner..."
./scripts/test_owner.sh $CONTRACT_ID "$NEW_OWNER"
echo "El owner sigue siendo correcto"

test_types
./scripts/log/log.sh $CONTRACT_ID
try_set_greeting_with_wrong_account