# rust grpc

from article:
https://dev.to/anshulgoyal15/a-beginners-guide-to-grpc-with-rust-3c7o

First run the server:
```bash
$ cargo run --bin server
```

Then run the client:
```bash
$ cargo run --bin client
```

## Project Structure
      + proto/
           say.proto (defining messages and services)
      + src/
           client
           server
           hello
      build.rs
        -> will compile proto/say.proto file and saveit in an OUT_DIR
           and add an environment variable OUT_DIR which is available
           at build time so that we can use it later in our code.