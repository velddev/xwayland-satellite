# Benchmark results

Trees:

- `stable-primary-17c0827`: 17c0827 (17c08273be), 20 rounds
- `stable-primary`: 17746c1 (17746c1e4f), 20 rounds
- `main`: b424e5e (b424e5e44f), 20 rounds
- `main-again`: b424e5e (b424e5e44f), 20 rounds

Baseline `stable-primary-17c0827`. Each sample is one process's median over its runs; tables give the median, p10 and p90 over rounds and the change of the median vs the baseline with a bootstrap 95% CI. A change is marked slower/faster only when its whole CI lies beyond the A/A noise floor and beyond 2%.

## Overview: change of the median vs baseline (wall / satellite CPU)

| benchmark | ops | `stable-primary-17c0827` wall, sat CPU (us) | `stable-primary` | `main` | `main-again` |
|---|---|---|---|---|---|
| barrier_only | 500 | 80.6, 43.4 | -7.6% / -7.4% | -2.7% / -1.4% | -3.4% / -1.9% |
| configure_position | 5000 | 12.1, 7.0 | -1.5% / +0.4% | -0.3% / -0.3% | +2.0% / +0.9% |
| configure_position_size | 1000 | 66.3, 23.6 | -4.6% / -6.0% | +1.2% / -2.2% | -2.3% / -2.2% |
| configure_position_sync | 1000 | 44.9, 22.2 | -5.9% / -3.8% | -1.9% / +1.2% | -3.0% / -1.5% |
| focus_m2 | 2000 | 41.5, 22.3 | +3.8% / +0.4% | +16.4% / +8.8% | +15.7% / +10.2% |
| focus_m200 | 2000 | 43.4, 23.3 | +0.7% / -0.6% | -10.3% / -9.4% | -4.2% / -5.9% |
| focus_m3_u1000 | 2000 | 40.2, 22.1 | -3.5% / -2.8% | +11.8% / +4.5% | +9.2% / +0.2% |
| focus_m50 | 2000 | 49.5, 25.4 | -20.6% / -16.6% | -13.0% / -11.3% | -8.0% / -11.0% |
| fullscreen_redundant | 500 | 107.3, 59.1 | -6.0% / -4.5% | -4.8% / -3.6% | -8.1% / -6.8% |
| fullscreen_toggle | 2000 | 27.5, 16.3 | -2.6% / -2.0% | -5.5% / -7.2% | -0.4% / -0.9% |
| fullscreen_toggle_wait | 500 | 39.7, 27.3 | -1.8% / -0.9% | +0.3% / -0.1% | -0.2% / +0.4% |
| map_churn | 200 | 308.9, 168.4 | -3.9% / -1.6% | -2.1% / -2.2% | -0.2% / -1.5% |
| output_enter_m200 | 2000 | 18.9, 10.7 | +7.0% / -0.5% | +281.1% **slower** / +205.3% **slower** | +241.0% **slower** / +190.5% **slower** |
| output_enter_m3 | 2000 | 21.3, 11.7 | -5.9% / -7.4% | +260.7% **slower** / +216.2% **slower** | +206.8% **slower** / +173.3% **slower** |
| output_enter_m3_u1000 | 2000 | 31.5, 15.0 | +0.7% / +1.0% | +219.7% **slower** / +176.5% **slower** | +200.8% **slower** / +141.1% **slower** |

## wall time per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| barrier_only | `stable-primary-17c0827` | 80.61 | 69.69 | 90.56 |  |  |  |
|  | `stable-primary` | 74.51 | 69.54 | 94.97 | -7.6% | -10.8% .. +11.4% | within noise |
|  | `main` | 78.40 | 68.10 | 106.57 | -2.7% | -10.8% .. +11.0% | within noise |
|  | `main-again` | 77.86 | 68.80 | 94.44 | -3.4% | -9.7% .. +11.0% | within noise |
| configure_position | `stable-primary-17c0827` | 12.11 | 11.45 | 13.03 |  |  |  |
|  | `stable-primary` | 11.93 | 11.53 | 15.56 | -1.5% | -5.6% .. +5.7% | within noise |
|  | `main` | 12.07 | 11.37 | 17.14 | -0.3% | -5.9% .. +9.8% | within noise |
|  | `main-again` | 12.36 | 11.58 | 13.93 | +2.0% | -3.4% .. +8.4% | within noise |
| configure_position_size | `stable-primary-17c0827` | 66.35 | 59.47 | 73.98 |  |  |  |
|  | `stable-primary` | 63.31 | 58.50 | 72.53 | -4.6% | -10.6% .. +4.7% | within noise |
|  | `main` | 67.16 | 58.21 | 139.25 | +1.2% | -6.9% .. +9.4% | within noise |
|  | `main-again` | 64.81 | 58.19 | 71.70 | -2.3% | -9.4% .. +7.3% | within noise |
| configure_position_sync | `stable-primary-17c0827` | 44.95 | 40.78 | 47.05 |  |  |  |
|  | `stable-primary` | 42.29 | 39.45 | 51.44 | -5.9% | -10.8% .. +0.6% | within noise |
|  | `main` | 44.08 | 38.49 | 71.48 | -1.9% | -9.0% .. +10.4% | within noise |
|  | `main-again` | 43.59 | 39.84 | 48.46 | -3.0% | -8.7% .. +1.9% | within noise |
| focus_m2 | `stable-primary-17c0827` | 41.51 | 36.17 | 50.28 |  |  |  |
|  | `stable-primary` | 43.09 | 35.80 | 56.14 | +3.8% | -18.6% .. +22.1% | within noise |
|  | `main` | 48.30 | 34.35 | 61.10 | +16.4% | -10.1% .. +29.5% | within noise |
|  | `main-again` | 48.03 | 35.92 | 56.96 | +15.7% | -15.4% .. +27.1% | within noise |
| focus_m200 | `stable-primary-17c0827` | 43.44 | 37.17 | 59.80 |  |  |  |
|  | `stable-primary` | 43.75 | 36.92 | 53.21 | +0.7% | -18.5% .. +18.0% | within noise |
|  | `main` | 38.98 | 35.58 | 50.24 | -10.3% | -23.8% .. +8.8% | within noise |
|  | `main-again` | 41.63 | 36.15 | 53.08 | -4.2% | -19.6% .. +17.8% | within noise |
| focus_m3_u1000 | `stable-primary-17c0827` | 40.17 | 37.26 | 56.15 |  |  |  |
|  | `stable-primary` | 38.76 | 36.33 | 51.10 | -3.5% | -16.8% .. +17.7% | within noise |
|  | `main` | 44.92 | 35.49 | 51.83 | +11.8% | -11.1% .. +22.1% | within noise |
|  | `main-again` | 43.88 | 35.60 | 49.83 | +9.2% | -13.3% .. +19.3% | within noise |
| focus_m50 | `stable-primary-17c0827` | 49.53 | 37.48 | 61.25 |  |  |  |
|  | `stable-primary` | 39.35 | 36.38 | 50.45 | -20.6% | -31.0% .. +9.9% | within noise |
|  | `main` | 43.11 | 35.58 | 124.98 | -13.0% | -27.2% .. +11.6% | within noise |
|  | `main-again` | 45.55 | 35.43 | 54.97 | -8.0% | -24.4% .. +10.6% | within noise |
| fullscreen_redundant | `stable-primary-17c0827` | 107.28 | 95.19 | 139.59 |  |  |  |
|  | `stable-primary` | 100.80 | 88.67 | 109.40 | -6.0% | -12.8% .. +0.8% | within noise |
|  | `main` | 102.10 | 88.48 | 179.94 | -4.8% | -13.7% .. +6.8% | within noise |
|  | `main-again` | 98.57 | 89.08 | 115.44 | -8.1% | -14.4% .. +0.9% | within noise |
| fullscreen_toggle | `stable-primary-17c0827` | 27.48 | 24.18 | 30.67 |  |  |  |
|  | `stable-primary` | 26.75 | 25.28 | 29.10 | -2.6% | -7.6% .. +1.4% | within noise |
|  | `main` | 25.97 | 22.24 | 33.64 | -5.5% | -12.4% .. +2.7% | within noise |
|  | `main-again` | 27.36 | 25.68 | 31.90 | -0.4% | -5.8% .. +6.8% | within noise |
| fullscreen_toggle_wait | `stable-primary-17c0827` | 39.69 | 37.34 | 48.28 |  |  |  |
|  | `stable-primary` | 38.99 | 37.03 | 48.04 | -1.8% | -6.3% .. +2.8% | within noise |
|  | `main` | 39.80 | 36.89 | 55.31 | +0.3% | -5.9% .. +10.6% | within noise |
|  | `main-again` | 39.63 | 36.94 | 48.75 | -0.2% | -5.5% .. +12.8% | within noise |
| map_churn | `stable-primary-17c0827` | 308.94 | 292.61 | 350.16 |  |  |  |
|  | `stable-primary` | 296.74 | 288.37 | 384.79 | -3.9% | -10.9% .. +8.2% | within noise |
|  | `main` | 302.53 | 282.82 | 351.64 | -2.1% | -11.4% .. +4.4% | within noise |
|  | `main-again` | 308.45 | 279.04 | 388.27 | -0.2% | -8.8% .. +6.2% | within noise |
| output_enter_m200 | `stable-primary-17c0827` | 18.88 | 16.66 | 25.14 |  |  |  |
|  | `stable-primary` | 20.20 | 16.52 | 23.84 | +7.0% | -14.1% .. +23.0% | within noise |
|  | `main` | 71.94 | 57.77 | 93.28 | +281.1% | +218.6% .. +371.8% | slower |
|  | `main-again` | 64.36 | 57.33 | 90.67 | +241.0% | +192.7% .. +337.4% | slower |
| output_enter_m3 | `stable-primary-17c0827` | 21.27 | 16.13 | 30.57 |  |  |  |
|  | `stable-primary` | 20.02 | 16.16 | 21.81 | -5.9% | -22.9% .. +11.1% | within noise |
|  | `main` | 76.72 | 55.46 | 116.79 | +260.7% | +181.7% .. +319.5% | slower |
|  | `main-again` | 65.26 | 55.20 | 82.62 | +206.8% | +168.9% .. +290.2% | slower |
| output_enter_m3_u1000 | `stable-primary-17c0827` | 31.52 | 26.13 | 40.85 |  |  |  |
|  | `stable-primary` | 31.75 | 25.58 | 37.32 | +0.7% | -11.6% .. +17.1% | within noise |
|  | `main` | 100.75 | 75.38 | 106.33 | +219.7% | +188.5% .. +271.3% | slower |
|  | `main-again` | 94.81 | 74.86 | 99.72 | +200.8% | +149.4% .. +249.2% | slower |

## satellite CPU per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| barrier_only | `stable-primary-17c0827` | 43.36 | 38.71 | 49.32 |  |  |  |
|  | `stable-primary` | 40.16 | 38.42 | 47.71 | -7.4% | -13.1% .. +6.4% | within noise |
|  | `main` | 42.74 | 38.25 | 53.34 | -1.4% | -11.4% .. +7.7% | within noise |
|  | `main-again` | 42.52 | 38.43 | 47.74 | -1.9% | -10.9% .. +9.0% | within noise |
| configure_position | `stable-primary-17c0827` | 7.02 | 6.79 | 7.27 |  |  |  |
|  | `stable-primary` | 7.05 | 6.82 | 7.55 | +0.4% | -2.8% .. +2.6% | within noise |
|  | `main` | 7.00 | 6.80 | 8.74 | -0.3% | -3.4% .. +4.8% | within noise |
|  | `main-again` | 7.08 | 6.88 | 7.48 | +0.9% | -2.8% .. +4.3% | within noise |
| configure_position_size | `stable-primary-17c0827` | 23.65 | 21.07 | 24.73 |  |  |  |
|  | `stable-primary` | 22.22 | 20.95 | 25.55 | -6.0% | -10.8% .. +4.5% | within noise |
|  | `main` | 23.11 | 20.79 | 35.99 | -2.2% | -8.7% .. +8.3% | within noise |
|  | `main-again` | 23.13 | 20.82 | 24.75 | -2.2% | -8.7% .. +8.1% | within noise |
| configure_position_sync | `stable-primary-17c0827` | 22.18 | 20.72 | 23.17 |  |  |  |
|  | `stable-primary` | 21.35 | 20.57 | 24.04 | -3.8% | -6.2% .. -0.1% | within noise |
|  | `main` | 22.45 | 20.20 | 31.85 | +1.2% | -4.6% .. +6.1% | within noise |
|  | `main-again` | 21.85 | 20.78 | 23.97 | -1.5% | -3.8% .. +1.2% | within noise |
| focus_m2 | `stable-primary-17c0827` | 22.30 | 20.27 | 26.16 |  |  |  |
|  | `stable-primary` | 22.40 | 20.13 | 27.17 | +0.4% | -15.7% .. +16.3% | within noise |
|  | `main` | 24.27 | 19.43 | 30.26 | +8.8% | -9.6% .. +18.5% | within noise |
|  | `main-again` | 24.57 | 19.80 | 27.62 | +10.2% | -13.1% .. +17.6% | within noise |
| focus_m200 | `stable-primary-17c0827` | 23.26 | 20.87 | 28.90 |  |  |  |
|  | `stable-primary` | 23.12 | 20.77 | 26.45 | -0.6% | -15.1% .. +12.2% | within noise |
|  | `main` | 21.09 | 19.83 | 25.27 | -9.4% | -20.5% .. +2.5% | within noise |
|  | `main-again` | 21.89 | 20.04 | 25.75 | -5.9% | -17.2% .. +9.4% | within noise |
| focus_m3_u1000 | `stable-primary-17c0827` | 22.06 | 21.05 | 28.44 |  |  |  |
|  | `stable-primary` | 21.45 | 20.63 | 26.50 | -2.8% | -13.1% .. +15.3% | within noise |
|  | `main` | 23.05 | 19.63 | 25.37 | +4.5% | -12.0% .. +12.6% | within noise |
|  | `main-again` | 22.11 | 19.80 | 24.64 | +0.2% | -12.9% .. +9.8% | within noise |
| focus_m50 | `stable-primary-17c0827` | 25.39 | 20.87 | 30.65 |  |  |  |
|  | `stable-primary` | 21.18 | 20.29 | 25.73 | -16.6% | -25.2% .. +10.9% | within noise |
|  | `main` | 22.53 | 19.78 | 37.71 | -11.3% | -22.7% .. +7.8% | within noise |
|  | `main-again` | 22.59 | 20.01 | 28.84 | -11.0% | -21.6% .. +9.0% | within noise |
| fullscreen_redundant | `stable-primary-17c0827` | 59.09 | 51.87 | 75.76 |  |  |  |
|  | `stable-primary` | 56.44 | 49.94 | 60.25 | -4.5% | -9.5% .. +1.7% | within noise |
|  | `main` | 56.99 | 49.67 | 90.57 | -3.6% | -12.4% .. +7.4% | within noise |
|  | `main-again` | 55.09 | 49.95 | 61.32 | -6.8% | -12.7% .. -0.9% | within noise |
| fullscreen_toggle | `stable-primary-17c0827` | 16.32 | 14.57 | 17.59 |  |  |  |
|  | `stable-primary` | 15.98 | 14.98 | 16.98 | -2.0% | -5.5% .. +1.7% | within noise |
|  | `main` | 15.13 | 13.07 | 18.15 | -7.2% | -11.1% .. +1.7% | within noise |
|  | `main-again` | 16.17 | 14.93 | 17.37 | -0.9% | -6.2% .. +3.5% | within noise |
| fullscreen_toggle_wait | `stable-primary-17c0827` | 27.32 | 26.39 | 30.28 |  |  |  |
|  | `stable-primary` | 27.06 | 25.88 | 30.73 | -0.9% | -4.0% .. +2.4% | within noise |
|  | `main` | 27.30 | 25.90 | 34.96 | -0.1% | -4.1% .. +4.7% | within noise |
|  | `main-again` | 27.43 | 26.01 | 32.31 | +0.4% | -2.8% .. +5.6% | within noise |
| map_churn | `stable-primary-17c0827` | 168.41 | 163.73 | 193.01 |  |  |  |
|  | `stable-primary` | 165.72 | 161.92 | 191.38 | -1.6% | -7.8% .. +4.6% | within noise |
|  | `main` | 164.71 | 157.13 | 192.99 | -2.2% | -8.8% .. +3.4% | within noise |
|  | `main-again` | 165.87 | 156.13 | 199.54 | -1.5% | -8.1% .. +2.7% | within noise |
| output_enter_m200 | `stable-primary-17c0827` | 10.73 | 9.67 | 14.27 |  |  |  |
|  | `stable-primary` | 10.68 | 9.52 | 12.50 | -0.5% | -12.8% .. +18.5% | within noise |
|  | `main` | 32.76 | 29.03 | 43.72 | +205.3% | +169.5% .. +289.6% | slower |
|  | `main-again` | 31.16 | 28.97 | 40.26 | +190.5% | +158.5% .. +263.5% | slower |
| output_enter_m3 | `stable-primary-17c0827` | 11.73 | 9.32 | 14.68 |  |  |  |
|  | `stable-primary` | 10.87 | 9.35 | 11.72 | -7.4% | -19.8% .. +6.6% | within noise |
|  | `main` | 37.10 | 28.81 | 47.66 | +216.2% | +155.9% .. +269.1% | slower |
|  | `main-again` | 32.07 | 29.19 | 39.73 | +173.3% | +148.3% .. +240.2% | slower |
| output_enter_m3_u1000 | `stable-primary-17c0827` | 15.04 | 12.69 | 18.68 |  |  |  |
|  | `stable-primary` | 15.19 | 12.72 | 16.98 | +1.0% | -11.3% .. +17.9% | within noise |
|  | `main` | 41.58 | 32.27 | 44.19 | +176.5% | +159.9% .. +218.4% | slower |
|  | `main-again` | 36.27 | 32.21 | 42.57 | +141.1% | +119.9% .. +203.9% | slower |

## Xwayland CPU per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| barrier_only | `stable-primary-17c0827` | 49.86 | 44.53 | 55.50 |  |  |  |
|  | `stable-primary` | 46.60 | 44.39 | 58.76 | -6.5% | -8.3% .. +5.4% | within noise |
|  | `main` | 47.60 | 43.64 | 64.09 | -4.5% | -7.9% .. +7.1% | within noise |
|  | `main-again` | 48.61 | 44.22 | 55.91 | -2.5% | -6.9% .. +7.4% | within noise |
| configure_position | `stable-primary-17c0827` | 6.80 | 6.56 | 7.06 |  |  |  |
|  | `stable-primary` | 6.82 | 6.60 | 7.38 | +0.2% | -2.2% .. +2.5% | within noise |
|  | `main` | 6.75 | 6.58 | 8.55 | -0.7% | -3.1% .. +4.4% | within noise |
|  | `main-again` | 6.83 | 6.63 | 7.22 | +0.4% | -2.8% .. +3.5% | within noise |
| configure_position_size | `stable-primary-17c0827` | 53.42 | 48.43 | 57.15 |  |  |  |
|  | `stable-primary` | 51.16 | 47.41 | 57.70 | -4.2% | -10.2% .. +4.2% | within noise |
|  | `main` | 52.36 | 47.43 | 85.22 | -2.0% | -7.7% .. +6.5% | within noise |
|  | `main-again` | 52.20 | 47.37 | 56.76 | -2.3% | -7.7% .. +6.1% | within noise |
| configure_position_sync | `stable-primary-17c0827` | 23.02 | 21.19 | 23.58 |  |  |  |
|  | `stable-primary` | 21.50 | 20.82 | 23.93 | -6.6% | -8.7% .. -0.5% | within noise |
|  | `main` | 22.91 | 20.30 | 32.16 | -0.4% | -8.1% .. +3.9% | within noise |
|  | `main-again` | 22.10 | 20.94 | 24.12 | -4.0% | -7.0% .. +1.9% | within noise |
| focus_m2 | `stable-primary-17c0827` | 23.65 | 21.77 | 28.34 |  |  |  |
|  | `stable-primary` | 23.91 | 21.85 | 29.93 | +1.1% | -15.4% .. +21.8% | within noise |
|  | `main` | 27.43 | 21.20 | 32.95 | +16.0% | -7.6% .. +24.9% | within noise |
|  | `main-again` | 27.75 | 21.75 | 29.51 | +17.3% | -10.6% .. +24.5% | within noise |
| focus_m200 | `stable-primary-17c0827` | 24.28 | 22.36 | 31.99 |  |  |  |
|  | `stable-primary` | 24.54 | 22.33 | 28.33 | +1.0% | -16.2% .. +15.1% | within noise |
|  | `main` | 23.32 | 21.71 | 29.11 | -4.0% | -19.6% .. +4.5% | within noise |
|  | `main-again` | 24.21 | 22.05 | 28.83 | -0.3% | -16.3% .. +16.1% | within noise |
| focus_m3_u1000 | `stable-primary-17c0827` | 22.88 | 22.03 | 30.43 |  |  |  |
|  | `stable-primary` | 22.57 | 21.76 | 28.65 | -1.4% | -11.6% .. +13.5% | within noise |
|  | `main` | 25.51 | 21.76 | 28.70 | +11.5% | -8.1% .. +21.6% | within noise |
|  | `main-again` | 24.00 | 21.83 | 27.93 | +4.9% | -8.8% .. +16.9% | within noise |
| focus_m50 | `stable-primary-17c0827` | 28.16 | 22.20 | 30.69 |  |  |  |
|  | `stable-primary` | 22.61 | 21.72 | 28.29 | -19.7% | -24.9% .. +12.6% | within noise |
|  | `main` | 24.49 | 21.80 | 39.87 | -13.1% | -22.4% .. +13.8% | within noise |
|  | `main-again` | 24.32 | 21.94 | 30.97 | -13.6% | -21.5% .. +15.3% | within noise |
| fullscreen_redundant | `stable-primary-17c0827` | 65.74 | 60.07 | 82.04 |  |  |  |
|  | `stable-primary` | 62.77 | 57.27 | 67.15 | -4.5% | -9.4% .. -0.5% | within noise |
|  | `main` | 62.88 | 57.50 | 100.18 | -4.4% | -10.0% .. +5.0% | within noise |
|  | `main-again` | 61.81 | 57.60 | 69.11 | -6.0% | -10.3% .. -0.2% | within noise |
| fullscreen_toggle | `stable-primary-17c0827` | 15.00 | 13.62 | 16.66 |  |  |  |
|  | `stable-primary` | 14.65 | 14.01 | 15.72 | -2.3% | -5.8% .. +0.7% | within noise |
|  | `main` | 13.97 | 12.67 | 17.15 | -6.8% | -10.1% .. +0.6% | within noise |
|  | `main-again` | 14.84 | 13.91 | 16.33 | -1.0% | -4.6% .. +3.1% | within noise |
| fullscreen_toggle_wait | `stable-primary-17c0827` | 19.05 | 18.37 | 21.11 |  |  |  |
|  | `stable-primary` | 18.70 | 18.00 | 21.29 | -1.8% | -4.8% .. +1.6% | within noise |
|  | `main` | 18.98 | 18.04 | 24.32 | -0.4% | -4.5% .. +4.5% | within noise |
|  | `main-again` | 18.96 | 18.25 | 22.16 | -0.5% | -3.6% .. +4.7% | within noise |
| map_churn | `stable-primary-17c0827` | 189.49 | 182.30 | 215.06 |  |  |  |
|  | `stable-primary` | 184.58 | 181.03 | 212.79 | -2.6% | -8.0% .. +4.5% | within noise |
|  | `main` | 182.75 | 176.16 | 217.20 | -3.6% | -9.2% .. +2.7% | within noise |
|  | `main-again` | 184.62 | 175.26 | 226.87 | -2.6% | -8.1% .. +2.3% | within noise |
| output_enter_m200 | `stable-primary-17c0827` | 11.35 | 10.49 | 14.93 |  |  |  |
|  | `stable-primary` | 11.65 | 10.40 | 14.35 | +2.7% | -13.3% .. +22.5% | within noise |
|  | `main` | 41.87 | 37.93 | 53.46 | +269.1% | +210.4% .. +356.8% | slower |
|  | `main-again` | 40.23 | 37.42 | 51.26 | +254.6% | +199.4% .. +338.6% | slower |
| output_enter_m3 | `stable-primary-17c0827` | 12.24 | 9.08 | 13.86 |  |  |  |
|  | `stable-primary` | 10.37 | 9.19 | 12.47 | -15.3% | -24.2% .. +6.7% | within noise |
|  | `main` | 46.08 | 35.21 | 55.62 | +276.5% | +198.9% .. +355.4% | slower |
|  | `main-again` | 38.81 | 35.48 | 47.95 | +217.1% | +191.1% .. +312.8% | slower |
| output_enter_m3_u1000 | `stable-primary-17c0827` | 20.73 | 17.30 | 24.89 |  |  |  |
|  | `stable-primary` | 20.79 | 16.75 | 23.08 | +0.3% | -10.5% .. +17.0% | within noise |
|  | `main` | 64.09 | 51.95 | 66.57 | +209.1% | +189.2% .. +257.9% | slower |
|  | `main-again` | 57.91 | 51.96 | 65.95 | +179.3% | +155.8% .. +244.9% | slower |

