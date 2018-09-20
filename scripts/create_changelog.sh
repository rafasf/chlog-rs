# This script takes care of testing your crate

set -ex

main() {
    target/release/chlog \
        --repository . \
        --range $(git describe --abbrev=0 --tags)..${TRAVIS_COMMIT}

    cat CHANGELOG.md
}

main
