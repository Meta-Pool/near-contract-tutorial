if [ "$#" -ne 1 ]; then
    echo "Para testear el saludo se requiere el nombre del contrato"
    exit 1
fi

CONTRACT_NAME=$1
echo "Calling log. No testing required, but check transaction on 'https://testnet.nearblocks.io/address/$CONTRACT_NAME'"
near view $CONTRACT_NAME log