# Recommendation
The following is not necessary and you should be able to work properly without following this, but we recommend to avoid implementing the `Default` trait since in most cases we will not have a Default contract, but we will have some initialization params. So for this case, check how we change the Default trait for a simple implementation using the `init` macro and define the `new` function on the `src/lib.rs` file.

We will also change the contract name for a more logic one.

In case you'd rather still use the Default, we recommend for the purpose of this tutorial to still do it and try doing the same with just the `Default` trait, since all the following code will have this change done.

We will also add to our Cargo.toml file into our dependencies the following line `borsh = "1.5.1"`. Borsh means `Binary Object Representation Serializer for Hashing` and will help us serialize and deserialize objects in order to improve storing data on the blockchain and recovering it. We want to do this for faster execution time and save storage cost.

We will also need to adapt our tests for this new structure, but the code contract will remain the same way.

After making all the fixes, we will need to build our project again by running `cargo build` and checking we have no error.