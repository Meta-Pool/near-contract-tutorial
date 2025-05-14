if [ "$#" -ne 1 ]; then
    echo "Para testear el saludo se requiere el nombre del contrato"
    exit 1
fi

CONTRACT_NAME=$1
RESULT=$(near view $CONTRACT_NAME return_u128_badly '{}' | awk 'NR==2 {print $0}')
if [ $RESULT == 4.2e+31 ]; then
    # echo "Si"
    exit 0
else
    # echo "No"
    exit 1
fi
