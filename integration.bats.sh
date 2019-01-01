#!/usr/bin/env bats

check_env() {
    if [ -n "${CARGO_LIPO_TEST_DIRECT_CALL+x}" ]; then
        export CARGO_LIPO="cargo-lipo lipo --color=always"
        return
    fi

    local CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"

    if [ -e "$CARGO_HOME/bin/cargo-lipo" ]; then
        echo 'ERROR: cargo-lipo exists in $CARGO_HOME/bin. This is a problem because Cargo'
        echo 'ERROR: will always search that directory first. Either remove that file or'
        echo 'ERROR: define $CARGO_LIPO_TEST_DIRECT_CALL'
        return 1
    fi

    export CARGO_LIPO="cargo lipo"
}

check_archs() {
    local actual="$(
        lipo -info "$2" |\
            # Get stuff after the last colon
            rev | cut -d':' -f1 | rev |\
            # Convert to one arch per line
            tr ' ' '\n' | grep -v '^$' |\
            # Sort and concat with comma
            sort | paste -sd',' -)"
    if [ "$1" != "$actual" ]; then
        echo "Expected: [$1], actual: [$actual]"
        return 1
    fi
}

setup() {
    cd "$BATS_TEST_DIRNAME/testdata"
    rm -rf {simple,workspace}/target

    check_env

    cargo build --color=always --manifest-path "$BATS_TEST_DIRNAME/Cargo.toml"
    export PATH="$BATS_TEST_DIRNAME/target/debug:$PATH"
}

@test "build simple in directory" {
    (cd simple && ${CARGO_LIPO})
    check_archs arm64,x86_64 simple/target/universal/debug/libsimple.a
}

@test "build simple in subdirectory" {
    (cd simple/src && ${CARGO_LIPO})
    check_archs arm64,x86_64 simple/target/universal/debug/libsimple.a
}

@test "build simple with --manifest-path" {
    ${CARGO_LIPO} --manifest-path simple/Cargo.toml
    check_archs arm64,x86_64 simple/target/universal/debug/libsimple.a
}

@test "build simple with --release" {
    ${CARGO_LIPO} --release --manifest-path simple/Cargo.toml
    check_archs arm64,x86_64 simple/target/universal/release/libsimple.a
}

@test "build simple with --targets" {
    ${CARGO_LIPO} --targets aarch64-apple-ios --manifest-path simple/Cargo.toml
    check_archs arm64 simple/target/universal/debug/libsimple.a
}

@test "build single project from workspace" {
    ${CARGO_LIPO} -p static1 --manifest-path workspace/Cargo.toml
    check_archs arm64,x86_64 workspace/target/universal/debug/libstatic1.a
}

@test "build project with build script" {
    ${CARGO_LIPO} -p static2build --manifest-path workspace/Cargo.toml
    check_archs arm64,x86_64 workspace/target/universal/debug/libstatic2build.a
}

@test "build project with multiple targets" {
    ${CARGO_LIPO} -p static3bin --manifest-path workspace/Cargo.toml
    check_archs arm64,x86_64 workspace/target/universal/debug/libstatic3bin.a
}

@test "build all from workspace" {
    ${CARGO_LIPO} --manifest-path workspace/Cargo.toml
    check_archs arm64,x86_64 workspace/target/universal/debug/libstatic1.a
    check_archs arm64,x86_64 workspace/target/universal/debug/libstatic2build.a
    check_archs arm64,x86_64 workspace/target/universal/debug/libstatic3bin.a
}
