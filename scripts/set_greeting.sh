if [ "$#" -ne 3 ]; then
    echo "Para testear el saludo se requiere el nombre del contrato, el saludo deseado y la cuenta desde la cual se llama al contrato"
    exit 1
fi

CONTRACT_NAME=$1
DESIRED_GREETING=$2
CALLING_ACCOUNT_ID=$3
near call $CONTRACT_NAME set_greeting '{"greeting": "'"$DESIRED_GREETING"'"}' --accountId $CALLING_ACCOUNT_ID --depositYocto 1