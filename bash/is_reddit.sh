#!/bin/bash

for i in $(cat instances.json| jq '.instances[].url'); do
    curl $i
done
