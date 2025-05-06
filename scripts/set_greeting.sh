if [ "$#" -ne 2 ]; then
    echo "Para testear el saludo se requiere el nombre del contrato y el saludo deseado"
    exit 1
fi

CONTRACT_NAME=$1
DESIRED_GREETING=$2
near call $CONTRACT_NAME set_greeting '{"greeting": "'"$DESIRED_GREETING"'"}' --accountId $CONTRACT_NAME