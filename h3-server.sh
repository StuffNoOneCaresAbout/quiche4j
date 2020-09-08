#!/usr/bin/env bash
set -eu -o pipefail

java \
    -Djava.library.path=src/main/rust/quiche_jni/target/debug/ \
    -cp target/quiche4j-0.1.0-SNAPSHOT.jar \
    io.quiche4j.examples.H3Server $1