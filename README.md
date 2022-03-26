# docker-funnies
Trying to make a small distroless docker image that can run a rust application

## Progress
| Export  | Total size | Comment |
| --------- | ----- | -----------------------------------------------------------------------|
| Export 1  | 28MB  | Initial image with dynamically linked libc (gcr.io/distroless/cc)      |
| Export 2  | 7.6MB | Statically linked with musl libc                                       |
| Export 3  | 5.6MB | Compiled with (codegen-units=1 panic="abort" opt-level="z" lto="thin") |
| Export 4  | 4.3MB | Stripped binary with -s option                                         |
