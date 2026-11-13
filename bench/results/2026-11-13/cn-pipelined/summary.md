# Benchmark results

Trees:

- `main`: b424e5e (b424e5e44f), 8 rounds
- `main-again`: b424e5e (b424e5e44f), 8 rounds
- `configure-notify`: c45fab6 (c45fab6e4c), 8 rounds
- `cn-pipelined`: bench/proto-configure-notify (459e0f012e), 8 rounds

Baseline `main`. A/A control `main-again` (same commit) sets the noise floor. Each sample is one process's median over its runs; tables give the median, p10 and p90 over rounds and the change of the median vs the baseline with a bootstrap 95% CI. A change is marked slower/faster only when its whole CI lies beyond the A/A noise floor and beyond 2%.

## Overview: change of the median vs baseline (wall / satellite CPU)

| benchmark | ops | `main` wall, sat CPU (us) | `main-again` | `configure-notify` | `cn-pipelined` |
|---|---|---|---|---|---|
| barrier_only | 500 | 80.8, 43.8 | +2.7% / +3.3% | -4.7% / -5.8% | -7.2% / -9.1% |
| configure_position | 5000 | 12.3, 7.0 | +2.5% / +0.4% | +257.0% **slower** / +200.4% **slower** | +20.2% / +6.8% |
| configure_position_size | 1000 | 66.9, 23.7 | +1.3% / -1.6% | +920.1% **slower** / +137.3% **slower** | +15.0% / +10.9% |
| configure_position_sync | 1000 | 44.6, 22.0 | +1.3% / +0.8% | +51.8% **slower** / +45.9% **slower** | +0.6% / -0.9% |
| focus_m2 | 2000 | 39.6, 20.6 | +13.7% / +10.2% | +24.7% / +21.4% | +1.5% / +1.3% |
| focus_m200 | 2000 | 44.9, 22.7 | -4.7% / -2.5% | +3.0% / +1.4% | -13.7% / -8.4% |
| focus_m3_u1000 | 2000 | 42.6, 22.0 | -5.9% / -5.8% | +13.7% / +12.0% | +4.7% / +2.9% |
| focus_m50 | 2000 | 45.3, 22.8 | -5.6% / -4.4% | -0.6% / +0.9% | -12.8% / -8.2% |
| fullscreen_redundant | 500 | 105.5, 58.7 | -7.6% / -8.9% | -5.3% / -3.9% | -4.4% / -5.0% |
| fullscreen_toggle | 2000 | 26.4, 15.3 | -1.5% / -1.3% | +1.9% / +1.1% | -6.8% / -5.3% |
| fullscreen_toggle_wait | 500 | 39.4, 27.0 | +2.3% / +1.1% | +3.0% / +2.0% | +2.1% / +1.1% |
| map_churn | 200 | 300.9, 164.5 | -0.7% / +0.2% | +3.9% / +1.5% | +5.3% / +1.0% |
| output_enter_m200 | 2000 | 63.0, 30.5 | -0.5% / -1.0% | +14.5% / +12.7% | +4.4% / +1.9% |
| output_enter_m3 | 2000 | 62.7, 31.1 | +11.6% / +9.6% | +2.8% / +1.3% | +10.3% / +8.1% |
| output_enter_m3_u1000 | 2000 | 88.7, 35.0 | -6.1% / -3.1% | -3.2% / +1.1% | -4.4% / -0.7% |

## wall time per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| barrier_only | `main` | 80.81 | 74.46 | 84.17 |  |  | A/A floor ±11.1% |
|  | `main-again` | 82.96 | 77.81 | 89.85 | +2.7% | -2.8% .. +11.6% | A/A |
|  | `configure-notify` | 77.00 | 72.49 | 79.38 | -4.7% | -9.9% .. +2.1% | within noise |
|  | `cn-pipelined` | 74.96 | 72.55 | 119.34 | -7.2% | -10.1% .. +5.7% | within noise |
| configure_position | `main` | 12.27 | 12.00 | 13.50 |  |  | A/A floor ±14.1% |
|  | `main-again` | 12.58 | 12.07 | 15.20 | +2.5% | -1.6% .. +14.3% | A/A |
|  | `configure-notify` | 43.81 | 33.74 | 45.19 | +257.0% | +180.5% .. +267.5% | slower |
|  | `cn-pipelined` | 14.75 | 13.38 | 23.08 | +20.2% | +9.8% .. +51.1% | within noise |
| configure_position_size | `main` | 66.90 | 61.70 | 91.66 |  |  | A/A floor ±10.1% |
|  | `main-again` | 67.74 | 64.81 | 76.76 | +1.3% | -9.8% .. +10.0% | A/A |
|  | `configure-notify` | 682.44 | 682.44 | 682.44 | +920.1% | +799.0% .. +997.0% | slower |
|  | `cn-pipelined` | 76.92 | 67.35 | 97.87 | +15.0% | -0.4% .. +32.6% | within noise |
| configure_position_sync | `main` | 44.58 | 41.12 | 53.00 |  |  | A/A floor ±9.4% |
|  | `main-again` | 45.15 | 42.33 | 47.07 | +1.3% | -7.9% .. +9.6% | A/A |
|  | `configure-notify` | 67.69 | 62.85 | 140.18 | +51.8% | +37.7% .. +94.3% | slower |
|  | `cn-pipelined` | 44.83 | 42.00 | 48.63 | +0.6% | -9.8% .. +11.9% | within noise |
| focus_m2 | `main` | 39.61 | 37.19 | 86.51 |  |  | A/A floor ±33.3% |
|  | `main-again` | 45.02 | 37.83 | 54.80 | +13.7% | -28.6% .. +33.3% | A/A |
|  | `configure-notify` | 49.41 | 41.93 | 98.41 | +24.7% | -18.4% .. +37.9% | within noise |
|  | `cn-pipelined` | 40.20 | 36.44 | 45.67 | +1.5% | -31.5% .. +12.7% | within noise |
| focus_m200 | `main` | 44.90 | 40.73 | 52.73 |  |  | A/A floor ±19.6% |
|  | `main-again` | 42.80 | 38.94 | 50.18 | -4.7% | -19.9% .. +13.9% | A/A |
|  | `configure-notify` | 46.25 | 37.92 | 83.57 | +3.0% | -21.7% .. +49.0% | within noise |
|  | `cn-pipelined` | 38.76 | 36.90 | 51.64 | -13.7% | -23.2% .. +14.5% | within noise |
| focus_m3_u1000 | `main` | 42.63 | 38.27 | 52.05 |  |  | A/A floor ±23.6% |
|  | `main-again` | 40.13 | 36.71 | 46.82 | -5.9% | -22.7% .. +11.3% | A/A |
|  | `configure-notify` | 48.47 | 37.97 | 92.02 | +13.7% | -16.4% .. +38.2% | within noise |
|  | `cn-pipelined` | 44.64 | 36.51 | 56.88 | +4.7% | -21.5% .. +25.4% | within noise |
| focus_m50 | `main` | 45.28 | 36.85 | 52.13 |  |  | A/A floor ±24.4% |
|  | `main-again` | 42.72 | 37.48 | 49.53 | -5.6% | -24.4% .. +19.7% | A/A |
|  | `configure-notify` | 45.02 | 37.65 | 92.05 | -0.6% | -23.7% .. +75.2% | within noise |
|  | `cn-pipelined` | 39.47 | 37.87 | 51.43 | -12.8% | -24.6% .. +14.9% | within noise |
| fullscreen_redundant | `main` | 105.51 | 95.82 | 108.64 |  |  | A/A floor ±12.2% |
|  | `main-again` | 97.47 | 93.49 | 110.53 | -7.6% | -12.1% .. +3.9% | A/A |
|  | `configure-notify` | 99.87 | 92.76 | 153.94 | -5.3% | -10.8% .. +22.1% | within noise |
|  | `cn-pipelined` | 100.86 | 95.73 | 119.83 | -4.4% | -8.6% .. +4.4% | within noise |
| fullscreen_toggle | `main` | 26.41 | 22.86 | 29.64 |  |  | A/A floor ±13.7% |
|  | `main-again` | 26.02 | 24.44 | 28.88 | -1.5% | -10.5% .. +13.7% | A/A |
|  | `configure-notify` | 26.90 | 22.59 | 31.66 | +1.9% | -14.1% .. +18.4% | within noise |
|  | `cn-pipelined` | 24.60 | 22.97 | 29.14 | -6.8% | -17.5% .. +13.8% | within noise |
| fullscreen_toggle_wait | `main` | 39.41 | 38.23 | 42.94 |  |  | A/A floor ±6.3% |
|  | `main-again` | 40.33 | 38.43 | 46.09 | +2.3% | -5.0% .. +6.3% | A/A |
|  | `configure-notify` | 40.60 | 39.02 | 73.10 | +3.0% | -5.4% .. +48.1% | within noise |
|  | `cn-pipelined` | 40.26 | 38.48 | 42.53 | +2.1% | -5.8% .. +7.4% | within noise |
| map_churn | `main` | 300.95 | 295.10 | 352.54 |  |  | A/A floor ±12.3% |
|  | `main-again` | 298.89 | 288.11 | 347.19 | -0.7% | -11.5% .. +7.0% | A/A |
|  | `configure-notify` | 312.68 | 288.65 | 396.63 | +3.9% | -9.3% .. +25.5% | within noise |
|  | `cn-pipelined` | 316.75 | 292.30 | 328.86 | +5.3% | -8.7% .. +7.8% | within noise |
| output_enter_m200 | `main` | 62.97 | 60.11 | 84.46 |  |  | A/A floor ±21.8% |
|  | `main-again` | 62.64 | 60.62 | 81.75 | -0.5% | -22.9% .. +18.5% | A/A |
|  | `configure-notify` | 72.07 | 60.25 | 99.17 | +14.5% | -16.5% .. +37.4% | within noise |
|  | `cn-pipelined` | 65.74 | 61.12 | 85.51 | +4.4% | -19.7% .. +23.8% | within noise |
| output_enter_m3 | `main` | 62.66 | 59.17 | 81.71 |  |  | A/A floor ±32.9% |
|  | `main-again` | 69.94 | 59.04 | 80.66 | +11.6% | -20.3% .. +32.9% | A/A |
|  | `configure-notify` | 64.42 | 60.66 | 84.64 | +2.8% | -19.6% .. +37.4% | within noise |
|  | `cn-pipelined` | 69.10 | 58.30 | 86.34 | +10.3% | -18.8% .. +36.2% | within noise |
| output_enter_m3_u1000 | `main` | 88.68 | 77.24 | 104.04 |  |  | A/A floor ±22.9% |
|  | `main-again` | 83.27 | 78.16 | 102.25 | -6.1% | -18.9% .. +22.9% | A/A |
|  | `configure-notify` | 85.81 | 78.62 | 112.80 | -3.2% | -19.1% .. +23.6% | within noise |
|  | `cn-pipelined` | 84.80 | 77.91 | 106.32 | -4.4% | -19.0% .. +25.3% | within noise |

## satellite CPU per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| barrier_only | `main` | 43.84 | 40.73 | 46.13 |  |  | A/A floor ±11.3% |
|  | `main-again` | 45.28 | 42.43 | 48.10 | +3.3% | -3.1% .. +10.9% | A/A |
|  | `configure-notify` | 41.32 | 40.09 | 42.78 | -5.8% | -10.5% .. +0.2% | within noise |
|  | `cn-pipelined` | 39.86 | 39.21 | 53.68 | -9.1% | -12.7% .. +7.4% | within noise |
| configure_position | `main` | 7.03 | 6.94 | 7.77 |  |  | A/A floor ±3.1% |
|  | `main-again` | 7.05 | 7.00 | 7.71 | +0.4% | -1.2% .. +3.1% | A/A |
|  | `configure-notify` | 21.11 | 16.79 | 21.75 | +200.4% | +141.9% .. +208.0% | slower |
|  | `cn-pipelined` | 7.50 | 7.05 | 10.02 | +6.8% | +0.6% .. +34.6% | within noise |
| configure_position_size | `main` | 23.71 | 21.29 | 28.08 |  |  | A/A floor ±9.7% |
|  | `main-again` | 23.33 | 21.70 | 25.03 | -1.6% | -9.4% .. +9.7% | A/A |
|  | `configure-notify` | 56.27 | 56.27 | 56.27 | +137.3% | +122.0% .. +164.3% | slower |
|  | `cn-pipelined` | 26.28 | 23.53 | 29.78 | +10.9% | -0.9% .. +24.7% | within noise |
| configure_position_sync | `main` | 21.96 | 21.02 | 25.72 |  |  | A/A floor ±6.4% |
|  | `main-again` | 22.14 | 21.27 | 22.97 | +0.8% | -6.1% .. +5.9% | A/A |
|  | `configure-notify` | 32.03 | 30.91 | 41.88 | +45.9% | +35.6% .. +79.0% | slower |
|  | `cn-pipelined` | 21.77 | 21.09 | 23.52 | -0.9% | -7.4% .. +7.3% | within noise |
| focus_m2 | `main` | 20.56 | 20.08 | 29.90 |  |  | A/A floor ±24.1% |
|  | `main-again` | 22.66 | 20.19 | 28.03 | +10.2% | -18.9% .. +24.1% | A/A |
|  | `configure-notify` | 24.96 | 22.05 | 30.70 | +21.4% | -10.0% .. +24.6% | within noise |
|  | `cn-pipelined` | 20.82 | 19.97 | 23.09 | +1.3% | -24.4% .. +8.7% | within noise |
| focus_m200 | `main` | 22.69 | 21.38 | 25.58 |  |  | A/A floor ±14.3% |
|  | `main-again` | 22.11 | 20.52 | 25.49 | -2.5% | -14.5% .. +10.7% | A/A |
|  | `configure-notify` | 23.01 | 20.48 | 30.94 | +1.4% | -14.6% .. +29.0% | within noise |
|  | `cn-pipelined` | 20.80 | 20.13 | 25.37 | -8.4% | -16.7% .. +10.5% | within noise |
| focus_m3_u1000 | `main` | 22.05 | 20.32 | 25.65 |  |  | A/A floor ±19.5% |
|  | `main-again` | 20.77 | 20.05 | 23.85 | -5.8% | -19.5% .. +9.9% | A/A |
|  | `configure-notify` | 24.69 | 20.10 | 30.85 | +12.0% | -14.6% .. +25.0% | within noise |
|  | `cn-pipelined` | 22.68 | 19.88 | 27.72 | +2.9% | -18.3% .. +20.0% | within noise |
| focus_m50 | `main` | 22.82 | 20.10 | 25.67 |  |  | A/A floor ±18.9% |
|  | `main-again` | 21.81 | 20.20 | 24.99 | -4.4% | -18.9% .. +17.4% | A/A |
|  | `configure-notify` | 23.01 | 20.39 | 34.92 | +0.9% | -18.4% .. +43.1% | within noise |
|  | `cn-pipelined` | 20.93 | 20.18 | 25.34 | -8.2% | -19.1% .. +11.5% | within noise |
| fullscreen_redundant | `main` | 58.72 | 54.25 | 60.01 |  |  | A/A floor ±13.3% |
|  | `main-again` | 53.48 | 51.23 | 59.42 | -8.9% | -13.3% .. +0.5% | A/A |
|  | `configure-notify` | 56.42 | 51.13 | 76.68 | -3.9% | -11.9% .. +13.4% | within noise |
|  | `cn-pipelined` | 55.78 | 52.11 | 67.16 | -5.0% | -9.7% .. +3.8% | within noise |
| fullscreen_toggle | `main` | 15.35 | 13.30 | 16.75 |  |  | A/A floor ±12.6% |
|  | `main-again` | 15.14 | 14.25 | 16.20 | -1.3% | -10.3% .. +12.1% | A/A |
|  | `configure-notify` | 15.52 | 13.25 | 17.75 | +1.1% | -12.8% .. +17.9% | within noise |
|  | `cn-pipelined` | 14.53 | 13.56 | 16.60 | -5.3% | -15.4% .. +14.5% | within noise |
| fullscreen_toggle_wait | `main` | 26.98 | 26.57 | 27.80 |  |  | A/A floor ±3.4% |
|  | `main-again` | 27.28 | 26.56 | 28.39 | +1.1% | -2.2% .. +3.6% | A/A |
|  | `configure-notify` | 27.52 | 26.73 | 37.27 | +2.0% | -2.2% .. +30.2% | within noise |
|  | `cn-pipelined` | 27.28 | 26.40 | 27.51 | +1.1% | -2.8% .. +2.6% | within noise |
| map_churn | `main` | 164.51 | 161.69 | 178.40 |  |  | A/A floor ±6.8% |
|  | `main-again` | 164.80 | 159.90 | 176.41 | +0.2% | -6.6% .. +4.3% | A/A |
|  | `configure-notify` | 166.98 | 159.16 | 194.91 | +1.5% | -6.5% .. +15.6% | within noise |
|  | `cn-pipelined` | 166.10 | 161.91 | 174.91 | +1.0% | -6.0% .. +5.7% | within noise |
| output_enter_m200 | `main` | 30.53 | 29.65 | 38.61 |  |  | A/A floor ±20.5% |
|  | `main-again` | 30.24 | 29.84 | 38.90 | -1.0% | -19.8% .. +16.1% | A/A |
|  | `configure-notify` | 34.42 | 29.53 | 41.44 | +12.7% | -13.2% .. +28.3% | within noise |
|  | `cn-pipelined` | 31.12 | 29.78 | 38.88 | +1.9% | -16.6% .. +18.1% | within noise |
| output_enter_m3 | `main` | 31.08 | 29.75 | 39.72 |  |  | A/A floor ±27.4% |
|  | `main-again` | 34.06 | 29.82 | 38.78 | +9.6% | -18.4% .. +27.6% | A/A |
|  | `configure-notify` | 31.48 | 30.10 | 39.43 | +1.3% | -17.1% .. +29.7% | within noise |
|  | `cn-pipelined` | 33.59 | 29.42 | 38.94 | +8.1% | -16.0% .. +27.5% | within noise |
| output_enter_m3_u1000 | `main` | 35.03 | 32.61 | 43.23 |  |  | A/A floor ±27.4% |
|  | `main-again` | 33.93 | 32.86 | 43.35 | -3.1% | -22.0% .. +27.4% | A/A |
|  | `configure-notify` | 35.41 | 32.85 | 45.02 | +1.1% | -22.3% .. +25.1% | within noise |
|  | `cn-pipelined` | 34.79 | 32.79 | 43.09 | -0.7% | -20.9% .. +27.1% | within noise |

## Xwayland CPU per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| barrier_only | `main` | 49.29 | 47.36 | 52.06 |  |  | A/A floor ±7.2% |
|  | `main-again` | 50.05 | 48.32 | 54.07 | +1.5% | -2.8% .. +7.2% | A/A |
|  | `configure-notify` | 47.87 | 45.45 | 48.87 | -2.9% | -8.2% .. +0.1% | within noise |
|  | `cn-pipelined` | 46.89 | 45.53 | 62.12 | -4.9% | -9.3% .. +1.2% | within noise |
| configure_position | `main` | 6.77 | 6.64 | 7.49 |  |  | A/A floor ±4.1% |
|  | `main-again` | 6.83 | 6.72 | 7.46 | +0.9% | -1.8% .. +4.1% | A/A |
|  | `configure-notify` | 25.22 | 19.67 | 25.76 | +272.4% | +192.5% .. +279.0% | slower |
|  | `cn-pipelined` | 8.36 | 7.89 | 11.60 | +23.5% | +16.6% .. +60.0% | slower |
| configure_position_size | `main` | 53.66 | 49.24 | 68.76 |  |  | A/A floor ±9.6% |
|  | `main-again` | 53.57 | 52.03 | 57.48 | -0.2% | -9.6% .. +8.3% | A/A |
|  | `configure-notify` | 121.73 | 121.73 | 121.73 | +126.8% | +105.3% .. +146.0% | slower |
|  | `cn-pipelined` | 60.46 | 53.04 | 69.45 | +12.7% | -2.1% .. +24.7% | within noise |
| configure_position_sync | `main` | 22.28 | 21.36 | 25.97 |  |  | A/A floor ±8.1% |
|  | `main-again` | 22.23 | 21.30 | 23.53 | -0.2% | -8.1% .. +7.7% | A/A |
|  | `configure-notify` | 36.09 | 34.40 | 45.33 | +62.0% | +46.0% .. +89.1% | slower |
|  | `cn-pipelined` | 22.96 | 22.31 | 25.76 | +3.1% | -5.5% .. +15.6% | within noise |
| focus_m2 | `main` | 22.80 | 22.17 | 32.15 |  |  | A/A floor ±27.4% |
|  | `main-again` | 24.86 | 22.09 | 30.98 | +9.1% | -19.2% .. +27.8% | A/A |
|  | `configure-notify` | 27.62 | 24.55 | 33.99 | +21.2% | -7.5% .. +28.6% | within noise |
|  | `cn-pipelined` | 22.65 | 21.88 | 25.22 | -0.7% | -23.1% .. +5.8% | within noise |
| focus_m200 | `main` | 25.07 | 23.23 | 29.08 |  |  | A/A floor ±19.4% |
|  | `main-again` | 24.64 | 22.65 | 27.73 | -1.7% | -19.6% .. +14.8% | A/A |
|  | `configure-notify` | 25.16 | 22.49 | 32.74 | +0.3% | -19.9% .. +25.8% | within noise |
|  | `cn-pipelined` | 23.02 | 22.19 | 28.83 | -8.2% | -20.7% .. +12.5% | within noise |
| focus_m3_u1000 | `main` | 24.70 | 22.36 | 28.58 |  |  | A/A floor ±20.0% |
|  | `main-again` | 22.71 | 21.91 | 26.29 | -8.0% | -20.2% .. +8.8% | A/A |
|  | `configure-notify` | 27.50 | 22.23 | 33.01 | +11.4% | -16.5% .. +27.6% | within noise |
|  | `cn-pipelined` | 25.59 | 21.79 | 30.05 | +3.6% | -19.7% .. +23.8% | within noise |
| focus_m50 | `main` | 25.36 | 22.12 | 28.90 |  |  | A/A floor ±21.6% |
|  | `main-again` | 23.76 | 22.21 | 28.08 | -6.3% | -21.6% .. +19.9% | A/A |
|  | `configure-notify` | 25.43 | 22.37 | 36.95 | +0.3% | -21.0% .. +42.1% | within noise |
|  | `cn-pipelined` | 22.79 | 22.15 | 28.32 | -10.1% | -22.2% .. +13.0% | within noise |
| fullscreen_redundant | `main` | 64.52 | 60.55 | 66.62 |  |  | A/A floor ±9.4% |
|  | `main-again` | 61.64 | 58.88 | 66.80 | -4.5% | -9.4% .. +3.0% | A/A |
|  | `configure-notify` | 63.27 | 59.02 | 87.12 | -1.9% | -7.6% .. +19.7% | within noise |
|  | `cn-pipelined` | 62.32 | 60.06 | 74.61 | -3.4% | -6.5% .. +3.4% | within noise |
| fullscreen_toggle | `main` | 14.38 | 12.64 | 15.39 |  |  | A/A floor ±11.5% |
|  | `main-again` | 14.25 | 13.54 | 14.92 | -0.9% | -8.1% .. +11.5% | A/A |
|  | `configure-notify` | 14.52 | 12.95 | 16.80 | +1.0% | -9.5% .. +15.1% | within noise |
|  | `cn-pipelined` | 13.69 | 12.89 | 15.52 | -4.8% | -13.3% .. +11.2% | within noise |
| fullscreen_toggle_wait | `main` | 18.77 | 18.33 | 19.17 |  |  | A/A floor ±3.7% |
|  | `main-again` | 18.78 | 18.46 | 19.84 | +0.0% | -2.2% .. +4.0% | A/A |
|  | `configure-notify` | 19.41 | 18.76 | 25.61 | +3.4% | -0.7% .. +29.4% | within noise |
|  | `cn-pipelined` | 18.91 | 18.32 | 19.11 | +0.7% | -2.0% .. +2.5% | within noise |
| map_churn | `main` | 186.06 | 180.57 | 199.99 |  |  | A/A floor ±7.6% |
|  | `main-again` | 183.34 | 179.45 | 198.35 | -1.5% | -7.6% .. +4.3% | A/A |
|  | `configure-notify` | 188.18 | 178.53 | 223.53 | +1.1% | -6.4% .. +17.6% | within noise |
|  | `cn-pipelined` | 189.10 | 181.64 | 196.46 | +1.6% | -5.5% .. +5.6% | within noise |
| output_enter_m200 | `main` | 39.72 | 38.69 | 50.98 |  |  | A/A floor ±21.2% |
|  | `main-again` | 39.22 | 38.88 | 49.90 | -1.3% | -17.2% .. +17.4% | A/A |
|  | `configure-notify` | 44.74 | 38.53 | 55.01 | +12.6% | -13.2% .. +29.6% | within noise |
|  | `cn-pipelined` | 40.29 | 38.74 | 50.18 | +1.4% | -15.9% .. +17.6% | within noise |
| output_enter_m3 | `main` | 37.68 | 36.28 | 48.09 |  |  | A/A floor ±27.5% |
|  | `main-again` | 41.46 | 36.12 | 47.53 | +10.0% | -16.7% .. +27.4% | A/A |
|  | `configure-notify` | 38.38 | 36.59 | 47.80 | +1.9% | -13.9% .. +28.8% | within noise |
|  | `cn-pipelined` | 41.62 | 35.88 | 48.50 | +10.4% | -14.6% .. +29.2% | within noise |
| output_enter_m3_u1000 | `main` | 56.90 | 52.79 | 66.72 |  |  | A/A floor ±21.0% |
|  | `main-again` | 54.12 | 52.62 | 66.23 | -4.9% | -16.9% .. +20.8% | A/A |
|  | `configure-notify` | 56.37 | 53.04 | 69.26 | -0.9% | -16.4% .. +19.2% | within noise |
|  | `cn-pipelined` | 55.42 | 53.08 | 66.54 | -2.6% | -15.4% .. +20.7% | within noise |

