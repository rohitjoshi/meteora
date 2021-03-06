# Meteora

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Meteora is a distributed key-value store written in [Rust](https://www.rust-lang.org/) built on top of [RocksDB](https://rocksdb.org/) and implemented by [The Raft Consensus Algorithm](https://raft.github.io/) and [The gRPC](https://grpc.io/).  
Achieves consensus across all the nodes, ensures every change made to the system is made to a quorum of nodes.  
Meteora makes easy for programmers to develop an applications with advanced features and high availability.


## Building Meteora

### Requirements

The following products are required to build Meteora:

- Rust: >= 1.42.0
- make: >= 3.81
- protoc >= 3.9.2
- rocksdb >= 5.18.3

Protobuf is needed only for code generation, `rust-protobuf` runtime does not use `protobuf` library.
Installl `protoc-gen-rust` program (which is `protoc` plugin) as follows"

```bash
% cargo install protobuf-codegen 
```


### Build

Build Meteora with the following command:

```bash
$ make build
```

When the build is successful, the binary file is output to the following directory:

```bash
$ ls ./bin
```



## Getting started

### Start in standalone mode (Single node cluster)

Running node in standalone mode is easy. You can start node with the following command:

```bash
$ ./bin/meteora start 1
```



## Setting data

You can set data with the following command:

```bash
$ ./bin/meteora set 1 "Meteora is a distributed key-value store."
```



## Getting data

You can get data with the following command:

```bash
$ ./bin/meteora get 1
```

You'll see the result of the above command like follows:

```text
Meteora is a distributed key-value store.
```


## Deleting data

You can delete data with the following command:

```bash
$ ./bin/meteora delete 1
```
