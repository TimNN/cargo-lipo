#!/usr/bin/env bats

check_env() {
    if [ -n "${CARGO_LIPO_TEST_DIRECT_CALL+x}" ]; then
        export CARGO_LIPO="cargo-lipo lipo --color=always"

        cat > xcode/.xcode-lipo <<EOF
#!/bin/bash

exec $BATS_TEST_DIRNAME/target/debug/cargo-lipo "\$@"
EOF

        chmod +x xcode/.xcode-lipo

        return
    fi

    local CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"

    if [ -e "$CARGO_HOME/bin/cargo-lipo" ]; then
        echo 'ERROR: cargo-lipo exists in $CARGO_HOME/bin. This is a problem because Cargo'
        echo 'ERROR: will always search that directory first. Either remove that file or'
        echo 'ERROR: define $CARGO_LIPO_TEST_DIRECT_CALL'
        return 1
    fi

    export CARGO_LIPO="cargo lipo --color=always"
}

check_archs() {
    # awk is used to trim a trailing space.
    local actual="$(lipo -archs "$2" | awk '{$1=$1};1' |  tr ' ' '\n' | sort | paste -sd',' -)"
    if [ "$1" != "$actual" ]; then
        echo "Expected: [$1], actual: [$actual]"
        return 1
    fi
}

xcode() {
    xcodebuild -workspace xcode/cargo-lipo-test.xcodeproj/project.xcworkspace -scheme cargo-lipo-test -configuration $2 -sdk "$SIM_SDK" $1
}

setup() {
    cd "$BATS_TEST_DIRNAME/testdata"
    rm -rf {simple,workspace}/target

    check_env

    cargo build --color=always --manifest-path "$BATS_TEST_DIRNAME/Cargo.toml"
    export PATH="$BATS_TEST_DIRNAME/target/debug:$PATH"

    export SIM_SDK="$(xcodebuild -showsdks | grep -o 'iphonesimulator[0-9.]*')"
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

@test "build simple with --profile=test-profile" {
    ${CARGO_LIPO} --profile=test-profile --manifest-path simple/Cargo.toml
    check_archs arm64,x86_64 simple/target/universal/test-profile/libsimple.a
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

# TODO: The tests below should only produce x86_64, but they also include arm64.

@test "xcode build debug for simulator" {
    skip "BROKEN: This test is currently broken, and I don't have the time to fix it."
    xcode "clean build" Debug
    check_archs arm64,x86_64 workspace/target/universal/debug/libstatic1.a
    check_archs arm64,x86_64 workspace/target/universal/debug/libstatic2build.a
    check_archs arm64,x86_64 workspace/target/universal/debug/libstatic3bin.a
}

@test "xcode build release for simulator" {
    skip "BROKEN: This test is currently broken, and I don't have the time to fix it."
    xcode "clean build" Release
    check_archs arm64,x86_64 workspace/target/universal/release/libstatic1.a
    check_archs arm64,x86_64 workspace/target/universal/release/libstatic2build.a
    check_archs arm64,x86_64 workspace/target/universal/release/libstatic3bin.a
}

@test "xcode install debug for simulator" {
    skip "BROKEN: This test is currently broken, and I don't have the time to fix it."
    xcode "clean install" Debug
    check_archs arm64,x86_64 workspace/target/universal/debug/libstatic1.a
    check_archs arm64,x86_64 workspace/target/universal/debug/libstatic2build.a
    check_archs arm64,x86_64 workspace/target/universal/debug/libstatic3bin.a
}
