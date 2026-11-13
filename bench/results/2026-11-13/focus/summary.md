# Benchmark results

Trees:

- `main`: b424e5e (b424e5e44f), 20 rounds
- `main-again`: b424e5e (b424e5e44f), 20 rounds
- `stable-primary`: 17c0827 (17c08273be), 20 rounds

Baseline `main`. A/A control `main-again` (same commit) sets the noise floor. Each sample is one process's median over its runs; tables give the median, p10 and p90 over rounds and the change of the median vs the baseline with a bootstrap 95% CI. A change is marked slower/faster only when its whole CI lies beyond the A/A noise floor and beyond 2%.

## Overview: change of the median vs baseline (wall / satellite CPU)

| benchmark | ops | `main` wall, sat CPU (us) | `main-again` | `stable-primary` |
|---|---|---|---|---|
| focus_m2 | 2000 | 47.9, 24.2 | -3.1% / -3.8% | -1.0% / -2.5% |
| focus_m200 | 2000 | 44.8, 22.9 | +6.7% / +5.9% | +7.0% / +8.6% |
| focus_m3_u1000 | 2000 | 46.6, 23.4 | -8.5% / -7.5% | +5.1% / +7.8% |
| focus_m50 | 2000 | 48.9, 24.3 | -1.7% / -2.5% | -2.4% / +0.0% |

## wall time per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| focus_m2 | `main` | 47.91 | 37.20 | 56.79 |  |  | A/A floor ±21.4% |
|  | `main-again` | 46.44 | 36.53 | 58.10 | -3.1% | -21.6% .. +17.1% | A/A |
|  | `stable-primary` | 47.45 | 38.94 | 50.93 | -1.0% | -13.5% .. +17.4% | within noise |
| focus_m200 | `main` | 44.76 | 37.18 | 52.72 |  |  | A/A floor ±37.7% |
|  | `main-again` | 47.76 | 37.99 | 77.68 | +6.7% | -11.5% .. +37.6% | A/A |
|  | `stable-primary` | 47.90 | 39.63 | 64.76 | +7.0% | -5.2% .. +25.2% | within noise |
| focus_m3_u1000 | `main` | 46.57 | 37.00 | 50.53 |  |  | A/A floor ±21.1% |
|  | `main-again` | 42.63 | 37.39 | 57.74 | -8.5% | -21.1% .. +18.2% | A/A |
|  | `stable-primary` | 48.95 | 40.29 | 56.16 | +5.1% | -3.1% .. +19.4% | within noise |
| focus_m50 | `main` | 48.91 | 37.20 | 58.30 |  |  | A/A floor ±21.4% |
|  | `main-again` | 48.06 | 38.58 | 53.31 | -1.7% | -8.4% .. +20.4% | A/A |
|  | `stable-primary` | 47.75 | 37.83 | 54.91 | -2.4% | -18.9% .. +19.8% | within noise |

## satellite CPU per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| focus_m2 | `main` | 24.23 | 20.12 | 26.59 |  |  | A/A floor ±16.6% |
|  | `main-again` | 23.30 | 20.00 | 27.38 | -3.8% | -16.6% .. +13.9% | A/A |
|  | `stable-primary` | 23.63 | 21.32 | 26.16 | -2.5% | -8.7% .. +15.0% | within noise |
| focus_m200 | `main` | 22.86 | 20.27 | 25.36 |  |  | A/A floor ±31.9% |
|  | `main-again` | 24.21 | 20.52 | 33.24 | +5.9% | -8.4% .. +32.1% | A/A |
|  | `stable-primary` | 24.84 | 21.75 | 33.78 | +8.6% | -3.4% .. +20.1% | within noise |
| focus_m3_u1000 | `main` | 23.36 | 19.98 | 25.07 |  |  | A/A floor ±16.4% |
|  | `main-again` | 21.61 | 20.13 | 26.96 | -7.5% | -16.6% .. +6.8% | A/A |
|  | `stable-primary` | 25.19 | 22.01 | 27.56 | +7.8% | -0.6% .. +19.1% | within noise |
| focus_m50 | `main` | 24.30 | 20.26 | 29.98 |  |  | A/A floor ±14.0% |
|  | `main-again` | 23.70 | 20.64 | 26.06 | -2.5% | -8.2% .. +13.6% | A/A |
|  | `stable-primary` | 24.31 | 20.88 | 26.97 | +0.0% | -11.7% .. +16.5% | within noise |

## Xwayland CPU per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| focus_m2 | `main` | 27.00 | 22.03 | 29.90 |  |  | A/A floor ±20.0% |
|  | `main-again` | 25.31 | 21.73 | 29.36 | -6.3% | -20.1% .. +9.5% | A/A |
|  | `stable-primary` | 25.14 | 22.56 | 28.24 | -6.9% | -15.8% .. +11.9% | within noise |
| focus_m200 | `main` | 24.53 | 22.45 | 29.05 |  |  | A/A floor ±28.0% |
|  | `main-again` | 27.36 | 22.63 | 35.67 | +11.5% | -9.8% .. +28.4% | A/A |
|  | `stable-primary` | 26.88 | 23.05 | 32.01 | +9.6% | -8.1% .. +19.2% | within noise |
| focus_m3_u1000 | `main` | 25.23 | 22.00 | 28.37 |  |  | A/A floor ±17.2% |
|  | `main-again` | 23.42 | 22.13 | 30.18 | -7.2% | -17.3% .. +12.9% | A/A |
|  | `stable-primary` | 26.37 | 22.79 | 29.24 | +4.5% | -6.7% .. +15.8% | within noise |
| focus_m50 | `main` | 27.76 | 22.10 | 30.20 |  |  | A/A floor ±16.1% |
|  | `main-again` | 25.62 | 22.67 | 28.84 | -7.7% | -13.7% .. +16.3% | A/A |
|  | `stable-primary` | 26.92 | 22.23 | 28.96 | -3.0% | -17.3% .. +18.9% | within noise |

