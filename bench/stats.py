#!/usr/bin/env python3
"""Summarise a bench/run.sh results directory as Markdown tables.

For every benchmark and tree: the median over rounds of the per-round medians, p10-p90 across
rounds, and the change of the median against the baseline (the first tree) with a bootstrap 95%
confidence interval. If a later tree has the baseline's commit (an A/A control), its interval sets
the noise floor for that benchmark and metric: a change counts only when its whole interval lies
beyond the floor (and beyond 2%).
"""

import random
import statistics
import sys
from collections import defaultdict
from pathlib import Path

METRICS = [
    # (column in BENCHMED after the op count, title, unit divisor, unit)
    (0, "wall time per op", 1000, "us"),
    (1, "satellite CPU per op", 1000, "us"),
    (2, "Xwayland CPU per op", 1000, "us"),
]
MIN_EFFECT = 0.02
BOOT = 4000


def pct(values, q):
    v = sorted(values)
    if len(v) == 1:
        return v[0]
    k = (len(v) - 1) * q
    lo = int(k)
    hi = min(lo + 1, len(v) - 1)
    return v[lo] + (v[hi] - v[lo]) * (k - lo)


def ratio_ci(base, other, rng):
    ratios = []
    for _ in range(BOOT):
        b = statistics.median(rng.choices(base, k=len(base)))
        o = statistics.median(rng.choices(other, k=len(other)))
        ratios.append(o / b)
    return pct(ratios, 0.025), pct(ratios, 0.975)


def main(out):
    out = Path(out)
    trees = [line.split("\t") for line in (out / "trees.tsv").read_text().splitlines() if line]
    labels = [t[0] for t in trees]
    commits = {t[0]: t[2] for t in trees}
    base = labels[0]
    aa = next((l for l in labels[1:] if commits[l] == commits[base]), None)

    # data[bench][label][metric] = [per-round medians]
    data = defaultdict(lambda: defaultdict(lambda: defaultdict(list)))
    ops = {}
    rounds = defaultdict(set)
    for line in (out / "results.tsv").read_text().splitlines():
        f = line.split("\t")
        label, rnd, _, bench, n = f[:5]
        values = [float(x) for x in f[5:]]
        ops[bench] = int(n)
        rounds[label].add(rnd)
        for m, *_ in METRICS:
            data[bench][label][m].append(values[m])

    rng = random.Random(1)
    print("# Benchmark results\n")
    print("Trees:\n")
    for label, ref, commit in trees:
        print(f"- `{label}`: {ref} ({commit[:10]}), {len(rounds[label])} rounds")
    print()
    print(
        f"Baseline `{base}`."
        + (f" A/A control `{aa}` (same commit) sets the noise floor." if aa else "")
        + " Each sample is one process's median over its runs; tables give the median, p10 and"
        " p90 over rounds and the change of the median vs the baseline with a bootstrap 95% CI."
        " A change is marked slower/faster only when its whole CI lies beyond the A/A noise"
        f" floor and beyond {MIN_EFFECT:.0%}.\n"
    )
    # verdicts[bench][label][metric] = (change, lo, hi, verdict)
    verdicts = defaultdict(lambda: defaultdict(dict))
    floors = defaultdict(dict)
    for bench in sorted(data):
        for m, *_ in METRICS:
            b_vals = data[bench][base][m]
            floor = 0.0
            if aa and data[bench][aa][m]:
                lo, hi = ratio_ci(b_vals, data[bench][aa][m], rng)
                floor = max(abs(lo - 1), abs(hi - 1))
            floors[bench][m] = floor
            for label in labels[1:]:
                vals = data[bench][label][m]
                if not vals:
                    continue
                lo, hi = ratio_ci(b_vals, vals, rng)
                change = statistics.median(vals) / statistics.median(b_vals) - 1
                limit = max(floor, MIN_EFFECT)
                verdict = "within noise"
                if label == aa:
                    verdict = "A/A"
                elif lo - 1 > limit:
                    verdict = "slower"
                elif 1 - hi > limit:
                    verdict = "faster"
                verdicts[bench][label][m] = (change, lo - 1, hi - 1, verdict)

    print("## Overview: change of the median vs baseline (wall / satellite CPU)\n")
    others = labels[1:]
    print("| benchmark | ops | `" + base + "` wall, sat CPU (us) | " + " | ".join(f"`{l}`" for l in others) + " |")
    print("|" + "---|" * (3 + len(others)))
    for bench in sorted(data):
        wall = statistics.median(data[bench][base][0]) / 1000
        sat = statistics.median(data[bench][base][1]) / 1000
        cells = []
        for label in others:
            parts = []
            for m in (0, 1):
                v = verdicts[bench][label].get(m)
                if v is None:
                    parts.append("-")
                    continue
                mark = {"slower": " **slower**", "faster": " **faster**"}.get(v[3], "")
                parts.append(f"{v[0]:+.1%}{mark}")
            cells.append(" / ".join(parts))
        print(f"| {bench} | {ops[bench]} | {wall:.1f}, {sat:.1f} | " + " | ".join(cells) + " |")
    print()

    for m, title, div, unit in METRICS:
        print(f"## {title} ({unit})\n")
        print("| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |")
        print("|---|---|---|---|---|---|---|---|")
        for bench in sorted(data):
            for label in labels:
                vals = data[bench][label][m]
                if not vals:
                    continue
                row = [
                    bench if label == base else "",
                    f"`{label}`",
                    f"{statistics.median(vals) / div:.2f}",
                    f"{pct(vals, 0.1) / div:.2f}",
                    f"{pct(vals, 0.9) / div:.2f}",
                ]
                if label == base:
                    row += ["", "", f"A/A floor ±{floors[bench][m]:.1%}" if aa else ""]
                else:
                    change, lo, hi, verdict = verdicts[bench][label][m]
                    row += [f"{change:+.1%}", f"{lo:+.1%} .. {hi:+.1%}", verdict]
                print("| " + " | ".join(row) + " |")
        print()

    suite = out / "suite.tsv"
    if suite.exists():
        times = defaultdict(list)
        failed = defaultdict(int)
        for line in suite.read_text().splitlines():
            label, _, ms, status = line.split("\t")
            times[label].append(int(ms) / 1000)
            failed[label] += status != "ok"
        print("## Full `cargo test --release` suite (s)\n")
        print("| tree | runs | median | p10-p90 | vs baseline | failed runs |")
        print("|---|---|---|---|---|---|")
        b = statistics.median(times[base]) if times[base] else float("nan")
        for label in labels:
            t = times[label]
            if not t:
                continue
            print(
                f"| `{label}` | {len(t)} | {statistics.median(t):.2f} | "
                f"{pct(t, 0.1):.2f}-{pct(t, 0.9):.2f} | "
                f"{statistics.median(t) / b - 1:+.1%} | {failed[label]} |"
            )
        print()


if __name__ == "__main__":
    main(sys.argv[1])
