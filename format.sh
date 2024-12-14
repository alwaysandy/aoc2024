for dir in */; do
    (cd "$dir" && cargo fmt)
done
