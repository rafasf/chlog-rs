# This script takes care of testing your crate

set -ex

main() {
    cargo run -- \
        --repository . \
        --range $(git describe --abbrev=0 --tags)..${TRAVIS_COMMIT}
}

main
