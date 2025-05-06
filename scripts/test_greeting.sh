if [ "$#" -ne 2 ]; then
    echo "Para testear el saludo se requiere el nombre del contrato y el saludo esperado"
    exit 1
fi

CONTRACT_NAME=$1
EXPECTED_GREETING=$2
RESULT=$(near view $CONTRACT_NAME get_greeting '{}' | awk -F"'" '{print $2}' | tr -d '\n')
# printf "STRING1: '%s'\n" "$RESULT"
# printf "STRING2: '%s'\n" "$EXPECTED_GREETING"
if [ "$RESULT" == "$EXPECTED_GREETING" ]; then
    # echo "Si"
    exit 0
else
    # echo "No"
    exit 1
fi
