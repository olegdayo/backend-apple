#!/bin/sh

index=1
while TRUE; do
    curl "http://localhost/frames/${index}?format=emoji&width=96&height=72"
    clear
    index=$((index+1))
done;
