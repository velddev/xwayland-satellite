# Benchmark results

Trees:

- `main`: b424e5e (b424e5e44f), 40 rounds
- `stable-primary`: 17746c1 (17746c1e4f), 40 rounds

Baseline `main`. Each sample is one process's median over its runs; tables give the median, p10 and p90 over rounds and the change of the median vs the baseline with a bootstrap 95% CI. A change is marked slower/faster only when its whole CI lies beyond the A/A noise floor and beyond 2%.

## Overview: change of the median vs baseline (wall / satellite CPU)

| benchmark | ops | `main` wall, sat CPU (us) | `stable-primary` |
|---|---|---|---|
| output_enter_m200 | 2000 | 74.0, 32.9 | -70.9% **faster** / -64.9% **faster** |
| output_enter_m3 | 2000 | 71.9, 32.9 | -72.5% **faster** / -68.3% **faster** |
| output_enter_m3_u1000 | 2000 | 94.7, 36.2 | -67.3% **faster** / -61.1% **faster** |

## wall time per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| output_enter_m200 | `main` | 74.05 | 65.69 | 89.10 |  |  |  |
|  | `stable-primary` | 21.53 | 18.37 | 24.13 | -70.9% | -73.9% .. -68.0% | faster |
| output_enter_m3 | `main` | 71.90 | 64.30 | 88.24 |  |  |  |
|  | `stable-primary` | 19.78 | 18.02 | 23.29 | -72.5% | -75.0% .. -69.2% | faster |
| output_enter_m3_u1000 | `main` | 94.69 | 83.43 | 107.86 |  |  |  |
|  | `stable-primary` | 30.92 | 28.42 | 34.94 | -67.3% | -69.2% .. -64.5% | faster |

## satellite CPU per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| output_enter_m200 | `main` | 32.85 | 31.25 | 40.31 |  |  |  |
|  | `stable-primary` | 11.54 | 10.05 | 12.79 | -64.9% | -69.5% .. -63.1% | faster |
| output_enter_m3 | `main` | 32.90 | 31.38 | 40.35 |  |  |  |
|  | `stable-primary` | 10.42 | 9.90 | 12.38 | -68.3% | -71.5% .. -64.7% | faster |
| output_enter_m3_u1000 | `main` | 36.18 | 34.45 | 44.44 |  |  |  |
|  | `stable-primary` | 14.07 | 13.45 | 16.55 | -61.1% | -66.7% .. -59.4% | faster |

## Xwayland CPU per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| output_enter_m200 | `main` | 42.58 | 40.56 | 52.41 |  |  |  |
|  | `stable-primary` | 12.63 | 11.05 | 14.43 | -70.3% | -74.0% .. -67.4% | faster |
| output_enter_m3 | `main` | 39.62 | 38.20 | 49.29 |  |  |  |
|  | `stable-primary` | 10.24 | 9.76 | 13.02 | -74.2% | -77.0% .. -69.5% | faster |
| output_enter_m3_u1000 | `main` | 57.61 | 55.25 | 67.82 |  |  |  |
|  | `stable-primary` | 19.35 | 18.20 | 22.33 | -66.4% | -70.3% .. -65.5% | faster |

