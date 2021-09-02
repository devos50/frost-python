# Bindings to the frost-dalek library

This repository contains bindings to invoke the functions in the `frost-dalek` library from Python, using the Pyo3 framework. **This code is highly experimental, please do not use it in production**.

Note that these bindings require a slightly modified version of the `frost-dalek` Rust library.
These modications can be found [here](https://github.com/devos50/frost-dalek/tree/bindings-compatibility).
Specifically, we had to remove some of the access modifiers in order to reconstruct some Rust types.
In addition, we had to remove lifetime specifiers since they are not compatible with `Pyclass` derives.

## Building

We recommend to use `maturin` in a `venv` to compile the bindings. Installing `maturin` can be done by running:

```
python3 -m venv .env
source .env/bin/activate
pip install maturin
```

Compiling the bindings can be done as follows:

```
maturin develop
```

## Testing

We provide a Python script that follows [the official usage guide](https://docs.rs/frost-dalek/0.2.3/frost_dalek), see `frost_test.py`.
Running this script after building the bindings should give the following output:

```
Done generating distributed key, group key: f299b914eb262e27813529df2c0b8642ae2a8e38b20895926ba7740a440f3d5b
Partial signature of Alice: d2ba666bcda25c1bddb8193b3eb1397af11948827ca3875c6953ef75ff7a3205
Partial signature of Carol: dcf3b6232f0d237f55306a5c17263af921957ecb94786bcc99d3de1ebcdaa60b
Resulting threshold Schnorr signature: da8fe01ac4c60a69bf34a5407fbe95aba970c742f38b3e0166f016677b829778c1da2732e24c6d425c4c8cf476dd945e13afc64d111cf3280327ce94bb55d900
Signature valid :)
```