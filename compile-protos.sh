#!/bin/sh -ex
protoc --rust_out=src src/proto/ironcoin_pb.proto
