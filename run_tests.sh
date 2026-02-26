#!/usr/bin/env bash
#
# run_tests.sh -- Build Qleany, generate example projects, and run their tests.
#
# Usage:
#   ./run_tests.sh                  Full suite: build qleany, generate Rust & C++/Qt
#                                   examples, build and test them, then clean up.
#   ./run_tests.sh -g|--generate    Only generate the examples (skip build & test
#                                   of the generated code). Output lands in
#                                   examples/rust/full/temp/ and examples/cpp-qt/full/temp/.
#   ./run_tests.sh --rust           Only run the Rust example (generate, build, test).
#   ./run_tests.sh --cpp-qt         Only run the C++/Qt example (generate, build, test).
#   ./run_tests.sh --no-cleanup      Keep generated temp/ directories after the run.
#   ./run_tests.sh --install-deps    Install Ubuntu packages first (Qt6, QCoro, Rust).
#
# Options can be combined, for example:
#   ./run_tests.sh --install-deps --generate
#   ./run_tests.sh --rust --generate    Generate only the Rust example (skip build & test)
#   ./run_tests.sh --cpp-qt -g          Generate only the C++/Qt example
#   ./run_tests.sh --rust --no-cleanup  Run Rust example and keep temp/ for inspection
#
# When neither --rust nor --cpp-qt is given, both examples are processed.
#
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$REPO_ROOT"

# -----------------------------------------------
# Parse arguments
# -----------------------------------------------
INSTALL_DEPS=false
GENERATE_ONLY=false
RUN_RUST=false
RUN_CPPQT=false
NO_CLEANUP=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --install-deps)
            INSTALL_DEPS=true
            shift
            ;;
        -g|--generate)
            GENERATE_ONLY=true
            shift
            ;;
        --rust)
            RUN_RUST=true
            shift
            ;;
        --cpp-qt)
            RUN_CPPQT=true
            shift
            ;;
        --no-cleanup)
            NO_CLEANUP=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --install-deps   Install Ubuntu dependencies (Qt6, QCoro, Rust) via apt/rustup"
            echo "  -g, --generate   Only generate the examples (skip build and test)"
            echo "  --rust           Only process the Rust example"
            echo "  --cpp-qt         Only process the C++/Qt example"
            echo "  --no-cleanup     Keep generated temp/ directories after the run"
            echo "  -h, --help       Show this help message"
            echo ""
            echo "When neither --rust nor --cpp-qt is given, both examples are processed."
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Run '$0 --help' for usage."
            exit 1
            ;;
    esac
done

# Default: run both when neither flag is set
if ! $RUN_RUST && ! $RUN_CPPQT; then
    RUN_RUST=true
    RUN_CPPQT=true
fi

# -----------------------------------------------
# Install dependencies (optional)
# -----------------------------------------------
if $INSTALL_DEPS; then
    echo "=== Installing Ubuntu dependencies ==="

    echo ""
    echo "--- Installing Qt6 and build tools ---"
    sudo apt-get update
    sudo apt-get install -y \
        build-essential \
        cmake \
        extra-cmake-modules \
        qt6-base-dev \
        qt6-declarative-dev \
        libgl1-mesa-dev

    echo ""
    echo "--- Installing QCoro ---"
    sudo apt-get install -y \
        qcoro-qt6-dev

    echo ""
    echo "--- Installing Rust (via rustup) ---"
    if command -v rustup &>/dev/null; then
        echo "rustup already installed, updating..."
        rustup update
    else
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        # shellcheck source=/dev/null
        source "$HOME/.cargo/env"
    fi

    echo ""
    echo "=== Dependencies installed ==="
fi

echo "=== Qleany Test Suite ==="

# -----------------------------------------------
# 1. Root project: check, build, and test
# -----------------------------------------------
echo ""
echo "--- Root: cargo check ---"
cargo check --workspace

echo ""
echo "--- Root: cargo build ---"
cargo build --workspace

if ! $GENERATE_ONLY; then
    echo ""
    echo "--- Root: cargo test ---"
    cargo test --workspace
fi

# -----------------------------------------------
# 2. Rust example (examples/rust/full)
# -----------------------------------------------
if $RUN_RUST; then
    echo ""
    echo "--- Rust example: generate ---"
    cd examples/rust/full
    "$REPO_ROOT/target/debug/qleany" gen --temp

    if ! $GENERATE_ONLY; then
        echo ""
        echo "--- Rust example: cargo check ---"
        cd temp
        cargo check --workspace

        echo ""
        echo "--- Rust example: cargo test ---"
        cargo test --workspace
    fi

    cd "$REPO_ROOT"
fi

# -----------------------------------------------
# 3. C++/Qt example (examples/cpp-qt/full)
# -----------------------------------------------
if $RUN_CPPQT; then
    echo ""
    echo "--- C++/Qt example: generate ---"
    cd examples/cpp-qt/full
    "$REPO_ROOT/target/debug/qleany" gen --temp

    if ! $GENERATE_ONLY; then
        echo ""
        echo "--- C++/Qt example: cmake configure ---"
        cd temp

        mkdir -p build
        cd build

        # Extract the BUILD_TESTS option prefix from the generated CMakeLists.txt
        BUILD_TESTS_OPT=$(grep -oP '\w+(?=_BUILD_TESTS)' ../CMakeLists.txt | head -1)
        cmake .. -D"${BUILD_TESTS_OPT}_BUILD_TESTS=ON"

        echo ""
        echo "--- C++/Qt example: cmake build ---"
        cmake --build . --target all -j"$(nproc)"

        echo ""
        echo "--- C++/Qt example: ctest ---"
        ctest --output-on-failure
    fi

    cd "$REPO_ROOT"
fi

# -----------------------------------------------
# 4. Clean up
# -----------------------------------------------
if ! $NO_CLEANUP; then
    echo ""
    echo "--- Clean up ---"
    if $RUN_RUST; then
        rm -rf examples/rust/full/temp
    fi
    if $RUN_CPPQT; then
        rm -rf examples/cpp-qt/full/temp
    fi
fi

echo ""
if $GENERATE_ONLY; then
    echo "=== Generation complete ==="
else
    echo "=== All tests passed ==="
fi
