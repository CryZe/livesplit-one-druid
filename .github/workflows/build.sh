set -ex

main() {
    local cargo=cross
    if [ "$SKIP_CROSS" = "skip" ]; then
        cargo=cargo
    fi
    local release_flag=""
    if [ "$IS_DEPLOY" = "true" ]; then
        release_flag="--profile max-opt"
    fi

    if [ -z "$FEATURES" ]; then
        FEATURE_FLAGS="--no-default-features"
    else
        FEATURE_FLAGS="--no-default-features --features $FEATURES"
    fi

    $cargo build --target $TARGET $release_flag $FEATURE_FLAGS
}

main
