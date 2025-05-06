if [ "$#" -ne 2 ]; then
    echo "Eliminar una cuenta de Near solo acepta 2 parámetros"
    exit 1
fi

ACCOUNT_ID_TO_DELETE=$1
BENEFICIARY_ACCOUNT_ID=$2

near delete-account $ACCOUNT_ID_TO_DELETE $BENEFICIARY_ACCOUNT_ID