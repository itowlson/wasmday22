# Creating the C files

For exports:

wit-bindgen c --export ./wit/console.wit --out-dir ./dotnet-guest/native/

For imports:

wit-bindgen c --import ./wit/random-thing.wit --out-dir ./dotnet-guest/native/
