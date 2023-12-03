default:
    @just --list

# Auto-format the source tree
fmt:
    treefmt

# Solve a given day '<day>'
solve *ARGS:
    cargo solve {{ARGS}}

# Runs 'cargo test'
test:
    cargo test

# Submit the solution for a given day '<day> <part> <answer>'
submit *ARGS:
    aoc submit -d {{ARGS}}

# Time runs for a given day '<day>'
time *ARGS:
    cargo solve {{ARGS}} --release --time

# Time all runs & Update README.md
timeall:
    cargo all --release --time