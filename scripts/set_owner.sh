if [ "$#" -ne 3 ]; then
    echo "Para testear el saludo se requiere el nombre del contrato, el nuevo owner deseado y la cuenta desde la cual se llama al contrato"
    exit 1
fi

CONTRACT_NAME=$1
NEW_OWNER=$2
CALLING_ACCOUNT_ID=$3
near call $CONTRACT_NAME set_owner '{"owner": "'"$NEW_OWNER"'"}' --accountId $CALLING_ACCOUNT_ID --depositYocto 1