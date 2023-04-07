set -ex

main() {
    local tag=$(git tag --points-at HEAD)
    local src=$(pwd) \
          stage=

    if [ "$OS_NAME" = "macOS-latest" ]; then
        stage=$(mktemp -d -t tmp)
    else
        stage=$(mktemp -d)
    fi

    if [ "$OS_NAME" = "ubuntu-latest" ]; then
        cp target/$TARGET/max-opt/livesplit-one $stage/LiveSplitOne
    elif [ "$OS_NAME" = "macOS-latest" ]; then
        cp target/$TARGET/max-opt/livesplit-one $stage/LiveSplitOne
    elif [ "$OS_NAME" = "windows-latest" ]; then
        cp target/$TARGET/max-opt/livesplit-one.exe $stage/LiveSplitOne.exe
    fi

    cd $stage
    if [ "$OS_NAME" = "windows-latest" ]; then
        7z a $src/livesplit-one-$tag-$RELEASE_TARGET.zip *
    else
        tar czf $src/livesplit-one-$tag-$RELEASE_TARGET.tar.gz *
    fi
    cd $src

    rm -rf $stage
}

main
