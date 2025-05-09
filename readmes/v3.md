# ¿Qué hay de nuevo?
En esta versión, agregaremos una serie de scripts para deployar el contrato. También llamaremos al contrato desde bash y desde [nearblocks](https://nearblocks.io/es). Finalmente, eliminaremos el contrato de la chain y aprenderemos algunos comandos básicos de near.

## Deployar
Por defecto, si no especificamos la red(network), estaremos interactuando contra testnet, así que se puede correr comandos sin riesgo de perder nada.

Para deployar un contrato e interactuar con el, necesitaremos añadir una librería de node, por lo que necesitaremos tener instalado npm. 
```
npm install near-cli -g
```
Luego de correr esto, podemos verificar que se haya instalado correctamente corriendo `near --version`. También podemos correr `near --help` para ver más comandos.

Una vez que lo tenemos instalado, podemos deployar el contrato corriendo `near deploy`. Requiere una cuenta y un archivo wasm, así que veamos como obtenerlos.

### Crear una cuenta
Una cuenta con nombre en testnet, termina con `.testnet`. En mainnet, termina con `.near`.
```
near create-account <your_account_id> --useFaucet
```
Esto debería generar una cuenta con 10 NEAR. Podemos verificarlo corriendo `near state <tu_cuenta>` y verificar las propiedades `formattedAmount` o `amount` removiendole 24 decimales.

### Crear un archivo wasm
Podemos conseguir esto corriendo
```
cargo build --target wasm32-unknown-unknown --release
```

### Deployando el contrato
Para deployar el contrato, hay que correr el comando
```
near deploy <your_account_id> <path_to_wasm_file>
```

A pesar de ser suficiente, recomendamos hacerlo de la siguiente manera
```
near deploy <your_account_id> <path_to_wasm_file> --initFunction new --initArgs '{"initial_greeting": "Hello"}'
```

¿Por qué recomendamos hacerlo así? Cuando llamamos a `deploy`, el contrato no va a tener estado hasta que llamemos a la función `new`. Así que cualquier individuo va a poder llamarlo colocando los parámetros que desee. Eventualmente vamos a tener contratos que van a tener cuentas con privilegios de administrador, así que queremos asegurarnos tener total control sobre esta cuenta y que no pase que sin darnos cuenta, un atacante tenga control sobre nuestro contrato. Aunque llamemos a la función `new` unos segundos más tarde, le damos la posibilidad de hacer esto.

No es especialmente grave si pasa y nos damos cuenta, porque podemos eliminar la cuenta y empezar fresco, pero más vale prevenir que curar.

#### Posibles inconvenientes
En el archivo `./rust-toolchain.toml` pueden tener la propiedad `channel` con valor `stable`. Si este es el caso, cambienló a `1.80.1`, recompilar y deployar

## Verificar tu contrato
### View
Luego de correr esto, deberías poder llamar a tu contrato. Por ejemplo, se puede correr
```
near view <your_account_id> <function>
```
y obtener el resultado. En el ejemplo como está arriba, el resultado debería ser `'Hello'`. 

### Call
También deberías poder modificar el contrato, llamando a una función de la siguiente manera
```
near call <your_account_id> <function> '{"arg1": "val1"}' --accountId <account_making_call>
```

### Nearblocks
También se puede llamar a estas funciones desde nearblocks yendo a [https://testnet.nearblocks.io/es/address/<your_account_id>?tab=contract](https://testnet.nearblocks.io/es/address/<your_account_id>?tab=contract).

## near-cli básico
### Credenciales
Cuando instalamos near-cli y creamos una cuenta, se generó un archivo en `~/.near-credentials/testnet/<your_account_id>.json` con 3 argumentos: account_id, public_key and private_key. No hace falta decir que la propiedad private_key debería ser guardada de forma segura.

Cuando se corre `near call` la propiedad --accountId seteada en <account_making_call> como arriba, near-cli buscará las credenciales de esta cuenta en la carpeta mencionada. Si no lo encuentra, no podrá firmar la transacción y retornará un error.

### Enviar near
Si querés enviar near, podés hacerlo corriendo `near send-near <sender> <receiver> <amount_in_near>`

### Login
Si te creaste una cuenta de otra forma y querés generar el archivo de credenciales, podés hacerlo corriendo `near login`. Esto abrirá `myNearWallet` y te guiará por el proceso poniendo tu clave privada o tu seed phrase. Una vez terminado, el archivo debería estar ahí.

### Eliminar la cuenta
Si queremos eliminar la cuenta para empezar fresco, se puede hacer corriendo `near delete-account <account_id> <beneficiary_id>`. Si se pone `testnet` como `beneficiary_id`, se retorna al faucet