set +x

sizes="1
10
100
1000
10000"

run() {
    # head = strip newline
    /usr/bin/time -f "%S,%U" 2>&1 cargo run -q --release --bin $1 -- $size | head -c -1
}

# make sure we're all built up
cargo build --release --workspace

echo "block kernel,block user,async kernel,async user"
for size in $sizes; do
    # time does annoying newline stuff
    block_entry=`run block`
    async_entry=`run async`
    # csv = ezpz
    echo "$block_entry,$async_entry"
done

