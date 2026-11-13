#!/usr/bin/env bash
# Compare satellite's overhead across git refs with tests/bench.rs. See BENCH.md.
#
#   bench/run.sh [-n REPS] [-s SUITE_REPS] [-f FILTER] [-o OUTDIR] [LABEL=]REF...
#
# Each ref gets a detached worktree under $BENCH_WORK (default target/bench/trees) with this
# checkout's tests/bench.rs copied in, and its own CARGO_TARGET_DIR. The bench binary of every
# tree then runs REPS times, interleaved (rotating order each round). -s also times the full
# `cargo test --release` suite SUITE_REPS times per tree. The first ref is the baseline; a later
# ref at the same commit is used as an A/A control to measure noise.
#
# Without refs it compares upstream main against the three PR branches:
#   main=b424e5e main-again=b424e5e stable-primary=upstream/stable-primary
#   configure-notify=upstream/configure-notify fullscreen-state=upstream/fullscreen-state
set -euo pipefail

here=$(cd "$(dirname "$0")/.." && pwd)
reps=20
suite_reps=0
filter=()
out=""
while getopts n:s:f:o:h opt; do
    case $opt in
        n) reps=$OPTARG ;;
        s) suite_reps=$OPTARG ;;
        f) filter=("$OPTARG") ;;
        o) out=$OPTARG ;;
        *) sed -n '2,15p' "$0"; exit 2 ;;
    esac
done
shift $((OPTIND - 1))
if [ $# -eq 0 ]; then
    set -- main=b424e5e main-again=b424e5e stable-primary=upstream/stable-primary \
        configure-notify=upstream/configure-notify fullscreen-state=upstream/fullscreen-state
fi

work=${BENCH_WORK:-$here/target/bench}
out=${out:-$work/results-$(date +%Y%m%d-%H%M%S)}
mkdir -p "$out" "$work/trees"
: > "$out/trees.tsv"

# Never talk to the session's compositor or X server.
run_env=(env -u DISPLAY -u WAYLAND_DISPLAY -u RUST_LOG)

labels=()
exes=()
for arg in "$@"; do
    if [[ $arg == *=* ]]; then
        label=${arg%%=*}
        ref=${arg#*=}
    else
        label=${arg//\//-}
        ref=$arg
    fi
    commit=$(git -C "$here" rev-parse --verify "$ref^{commit}")
    tree=$work/trees/$label
    if [ -e "$tree/.git" ]; then
        git -C "$tree" checkout -q --detach -f "$commit"
    else
        git -C "$here" worktree add -q --detach "$tree" "$commit"
    fi
    cp "$here/tests/bench.rs" "$tree/tests/bench.rs"
    echo "building $label ($ref = ${commit:0:7})" >&2
    if ! (cd "$tree" && CARGO_TARGET_DIR=$work/target-$label cargo test --release --test bench \
        --no-run --message-format=json 2> "$out/build-$label.log" > "$out/build-$label.json"); then
        cat "$out/build-$label.log" >&2
        exit 1
    fi
    exe=$(python3 -c '
import json, sys
for line in open(sys.argv[1]):
    m = json.loads(line)
    if m.get("reason") == "compiler-artifact" and m.get("executable") and m["target"]["name"] == "bench":
        print(m["executable"])' "$out/build-$label.json")
    printf '%s\t%s\t%s\n' "$label" "$ref" "$commit" >> "$out/trees.tsv"
    labels+=("$label")
    exes+=("$exe")
done

count=${#labels[@]}
for ((r = 1; r <= reps; r++)); do
    for ((k = 0; k < count; k++)); do
        i=$(((k + r - 1) % count))
        label=${labels[$i]}
        log=$out/raw-$label-$r.log
        echo "round $r/$reps: $label" >&2
        if ! (cd "$work/trees/$label" && "${run_env[@]}" timeout 900 "${exes[$i]}" \
            --ignored --test-threads 1 --nocapture "${filter[@]}") > "$log" 2>&1; then
            echo "  run failed, see $log" >&2
        fi
        grep -P '^BENCHMED\t' "$log" | sed "s/^/$label\t$r\t/" >> "$out/results.tsv" || true
    done
done

if ((suite_reps > 0)); then
    for label in "${labels[@]}"; do
        echo "building the test suite of $label" >&2
        (cd "$work/trees/$label" && CARGO_TARGET_DIR=$work/target-$label \
            cargo test --release --no-run > /dev/null 2>&1)
    done
    for ((r = 1; r <= suite_reps; r++)); do
        for ((k = 0; k < count; k++)); do
            i=$(((k + r - 1) % count))
            label=${labels[$i]}
            echo "suite round $r/$suite_reps: $label" >&2
            start=$(date +%s%N)
            status=ok
            (cd "$work/trees/$label" && CARGO_TARGET_DIR=$work/target-$label "${run_env[@]}" \
                timeout 900 cargo test --release > "$out/suite-$label-$r.log" 2>&1) || status=failed
            end=$(date +%s%N)
            printf '%s\t%s\t%s\t%s\n' "$label" "$r" "$(((end - start) / 1000000))" "$status" \
                >> "$out/suite.tsv"
        done
    done
fi

python3 "$here/bench/stats.py" "$out" | tee "$out/summary.md"
echo "results in $out" >&2
