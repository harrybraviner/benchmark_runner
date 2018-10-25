#! /bin/bash

DIR="$( cd "$( dirname "$BASH_SOURCE[0]" )" >/dev/null && pwd )"

mvn exec:java -f ${DIR} -Dexec.mainClass="braviner.harry.benchmarking.java_reader.SerialLineCounter" -Dexec.args="$*" -q
