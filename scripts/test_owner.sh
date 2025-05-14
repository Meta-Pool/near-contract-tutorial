if [ "$#" -ne 2 ]; then
    echo "Para testear el saludo se requiere el nombre del contrato y el owner esperado"
    exit 1
fi

CONTRACT_NAME=$1
EXPECTED_OWNER=$2
RESULT=$(near view $CONTRACT_NAME get_owner '{}' | awk -F"'" '{print $2}' | tr -d '\n')
if [ "$RESULT" == "$EXPECTED_OWNER" ]; then
    # echo "Si"
    exit 0
else
    # echo "No"
    exit 1
fi
