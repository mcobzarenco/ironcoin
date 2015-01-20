#!/bin/sh -ex
protoc --rust_out=src src/proto/simples_pb.proto
