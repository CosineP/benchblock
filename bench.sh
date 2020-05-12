#!/bin/bash

set +x

do_time() {
    # head = strip newline
    /usr/bin/time -f "%e,%S,%U" 2>&1 ./run.sh $1 $size | head -c -1
}

# make sure we're all built up
cargo build --release --workspace

if [ -n "$1" ]; then
    ./gen-file.sh $1
fi

echo "size,blocking (seconds),blocking (kernel time),blocking (user time),async (seconds),async (kernel time),async (user time)"
interval=1000
max=10000
#for size in {$interval..$max..$interval}; do
for size in {1000..10000..1000}; do
    block_entry=`do_time block`
    async_entry=`do_time async`
    # csv = ezpz
    echo "$size,$block_entry,$async_entry"
done

