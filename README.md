# Creating the C files

For exports:

wit-bindgen c --export ./wit/console.wit --out-dir ./dotnet-guest/native/

For imports:

wit-bindgen c --import ./wit/random-thing.wit --out-dir ./dotnet-guest/native/

# Building the host

```
cd host
cargo build --release
```

# Building the guest

```
cd dotnet-guest
dotnet build
```

# Testing

```
./host/target/release/host ./dotnet-guest/bin/Debug/net7.0/WasmDayDotnet.wasm
```
