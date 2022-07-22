
# Mongo Bug Test Package

`fix-core` is core package that provide existing runtime.

`fix-plugin` is plugin package that provide mongo plugin. It can be compiled as cdylib named `fix_plugin` or integrated with `fix-core` as a single executable.

`fix-main` is main executable that can be run with shared library which named `fix_plugin`.

`fix-integrated` is single executable that combine `fix-core` with `fix-plugin`.

# Reproduce the Problem
build release and run `.\fix-main.exe` command in `target\release` dir.
you will see: 
```
shared library loaded!
kernel created!
plugin init!
install mongo plugin!
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
run mongo api with temporary tokio runtime created in current shared library
now we are in async function
now we are in async function: client_options created
now we are in async function: client_options.app_name set
now we are in async function: client created
now we are in async function: database created
startup_log
instruments
now we are in async function: collection_name list finished
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
run mongo api with existing tokio runtime created in main executable
now we are in async function
now we are in async function: client_options created
now we are in async function: client_options.app_name set
now we are in async function: client created
now we are in async function: database created
thread '<unnamed>' panicked at 'cannot list collection names: Error { kind: ServerSelection { message: "Server selection timeout: No available servers. Topology: { Type: Unknown, Servers: [ { Address: 127.0.0.1:27017, Type: Unknown }, ] }" }, labels: {}, wire_version: None, source: None }', fix-plugin\src\lib.rs:63:79
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
# Bug Code
see `fix-plugin/src/lib.rs:63`.
it will report a exception like 
```
thread '<unnamed>' panicked at 'cannot list collection names: Error { kind: ServerSelection { message: "Server selection timeout: No available servers. Topology: { Type: Unknown, Servers: [ { Address: 127.0.0.1:27017, Type: Unknown }, ] }" }, labels: {}, wire_version: None, source: None }', fix-plugin\src\lib.rs:63:79
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

