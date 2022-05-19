#!/bin/bash

ASSET="$1" yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/best-apy/best-apy.ts --exit
