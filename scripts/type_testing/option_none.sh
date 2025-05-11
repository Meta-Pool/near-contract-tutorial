if [ "$#" -ne 1 ]; then
    echo "Para testear el saludo se requiere el nombre del contrato"
    exit 1
fi

CONTRACT_NAME=$1
RESULT=$(near view $CONTRACT_NAME return_option_none '{}' | awk 'NR>1 {print $0}')
EXPECTED=$(printf "%s" "null")
if [ "$RESULT" == "$EXPECTED" ]; then
    # echo "Si"
    exit 0
else
    # echo "No"
    exit 1
fi
