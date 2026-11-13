# Benchmark results

Trees:

- `main`: b424e5e (b424e5e44f), 20 rounds
- `main-again`: b424e5e (b424e5e44f), 20 rounds
- `stable-primary`: 17746c1 (17746c1e4f), 20 rounds
- `stable-primary-17c0827`: 17c0827 (17c08273be), 20 rounds
- `configure-notify`: 0ecb0fb (0ecb0fb8f0), 20 rounds
- `fullscreen-state`: fb0ed3b (fb0ed3b7f0), 20 rounds

Baseline `main`. A/A control `main-again` (same commit) sets the noise floor. Each sample is one process's median over its runs; tables give the median, p10 and p90 over rounds and the change of the median vs the baseline with a bootstrap 95% CI. A change is marked slower/faster only when its whole CI lies beyond the A/A noise floor and beyond 2%.

## Overview: change of the median vs baseline (wall / satellite CPU)

| benchmark | ops | `main` wall, sat CPU (us) | `main-again` | `stable-primary` | `stable-primary-17c0827` | `configure-notify` | `fullscreen-state` |
|---|---|---|---|---|---|---|---|
| barrier_only | 500 | 78.4, 42.7 | -0.7% / -0.5% | -5.0% / -6.0% | +2.8% / +1.4% | +4.6% / +5.3% | +6.0% / +5.5% |
| configure_position | 5000 | 12.1, 7.0 | +2.3% / +1.2% | -1.2% / +0.6% | +0.3% / +0.3% | +52.6% **slower** / +32.9% **slower** | -0.9% / +0.0% |
| configure_position_size | 1000 | 67.2, 23.1 | -3.5% / +0.1% | -5.7% / -3.9% | -1.2% / +2.3% | +13.4% / +14.8% | -6.5% / -4.4% |
| configure_position_sync | 1000 | 44.1, 22.5 | -1.1% / -2.7% | -4.0% / -4.9% | +2.0% / -1.2% | +8.8% / +2.5% | -3.1% / -3.5% |
| focus_m2 | 2000 | 48.3, 24.3 | -0.6% / +1.2% | -10.8% / -7.7% | -14.1% / -8.1% | -11.3% / -8.9% | -18.6% / -14.3% |
| focus_m200 | 2000 | 39.0, 21.1 | +6.8% / +3.8% | +12.2% / +9.7% | +11.4% / +10.3% | +2.1% / -0.9% | +5.6% / +2.2% |
| focus_m3_u1000 | 2000 | 44.9, 23.1 | -2.3% / -4.1% | -13.7% / -7.0% | -10.6% / -4.3% | -11.7% / -8.2% | -2.0% / +1.1% |
| focus_m50 | 2000 | 43.1, 22.5 | +5.7% / +0.3% | -8.7% / -6.0% | +14.9% / +12.7% | +8.7% / +5.1% | -5.9% / -5.0% |
| fullscreen_redundant | 500 | 102.1, 57.0 | -3.5% / -3.3% | -1.3% / -1.0% | +5.1% / +3.7% | +2.3% / +2.2% | +10.7% / +8.7% |
| fullscreen_toggle | 2000 | 26.0, 15.1 | +5.4% / +6.9% | +3.0% / +5.6% | +5.8% / +7.8% | +6.0% / +6.1% | +39.2% **slower** / +36.8% **slower** |
| fullscreen_toggle_wait | 500 | 39.8, 27.3 | -0.4% / +0.5% | -2.0% / -0.9% | -0.3% / +0.1% | -2.2% / -1.1% | +28.6% **slower** / +19.7% **slower** |
| map_churn | 200 | 302.5, 164.7 | +2.0% / +0.7% | -1.9% / +0.6% | +2.1% / +2.2% | +1.2% / +1.1% | +2.4% / +0.4% |
| output_enter_m200 | 2000 | 71.9, 32.8 | -10.5% / -4.9% | -71.9% **faster** / -67.4% **faster** | -73.8% **faster** / -67.2% **faster** | -4.4% / +1.0% | -14.4% / -7.9% |
| output_enter_m3 | 2000 | 76.7, 37.1 | -14.9% / -13.6% | -73.9% **faster** / -70.7% **faster** | -72.3% **faster** / -68.4% **faster** | -4.3% / -5.0% | -15.2% / -12.5% |
| output_enter_m3_u1000 | 2000 | 100.7, 41.6 | -5.9% / -12.8% | -68.5% **faster** / -63.5% **faster** | -68.7% **faster** / -63.8% **faster** | -7.0% / -9.6% | -6.5% / +0.5% |

## wall time per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| barrier_only | `main` | 78.40 | 68.10 | 106.57 |  |  | A/A floor ±11.0% |
|  | `main-again` | 77.86 | 68.80 | 94.44 | -0.7% | -6.9% .. +11.0% | A/A |
|  | `stable-primary` | 74.51 | 69.54 | 94.97 | -5.0% | -9.1% .. +10.1% | within noise |
|  | `stable-primary-17c0827` | 80.61 | 69.69 | 90.56 | +2.8% | -9.9% .. +12.3% | within noise |
|  | `configure-notify` | 81.98 | 68.66 | 136.80 | +4.6% | -2.2% .. +14.3% | within noise |
|  | `fullscreen-state` | 83.11 | 66.93 | 88.93 | +6.0% | -2.1% .. +15.2% | within noise |
| configure_position | `main` | 12.07 | 11.37 | 17.14 |  |  | A/A floor ±9.3% |
|  | `main-again` | 12.36 | 11.58 | 13.93 | +2.3% | -7.1% .. +9.3% | A/A |
|  | `stable-primary` | 11.93 | 11.53 | 15.56 | -1.2% | -9.8% .. +6.3% | within noise |
|  | `stable-primary-17c0827` | 12.11 | 11.45 | 13.03 | +0.3% | -9.4% .. +6.2% | within noise |
|  | `configure-notify` | 18.42 | 13.13 | 19.19 | +52.6% | +16.3% .. +59.0% | slower |
|  | `fullscreen-state` | 11.97 | 11.45 | 13.13 | -0.9% | -9.7% .. +5.0% | within noise |
| configure_position_size | `main` | 67.16 | 58.21 | 139.25 |  |  | A/A floor ±10.2% |
|  | `main-again` | 64.81 | 58.19 | 71.70 | -3.5% | -10.7% .. +6.6% | A/A |
|  | `stable-primary` | 63.31 | 58.50 | 72.53 | -5.7% | -11.9% .. +3.8% | within noise |
|  | `stable-primary-17c0827` | 66.35 | 59.47 | 73.98 | -1.2% | -8.8% .. +8.0% | within noise |
|  | `configure-notify` | 76.18 | 66.51 | 105.67 | +13.4% | +4.6% .. +25.1% | within noise |
|  | `fullscreen-state` | 62.79 | 57.79 | 82.41 | -6.5% | -12.0% .. +5.9% | within noise |
| configure_position_sync | `main` | 44.08 | 38.49 | 71.48 |  |  | A/A floor ±12.4% |
|  | `main-again` | 43.59 | 39.84 | 48.46 | -1.1% | -12.6% .. +6.7% | A/A |
|  | `stable-primary` | 42.29 | 39.45 | 51.44 | -4.0% | -16.0% .. +5.3% | within noise |
|  | `stable-primary-17c0827` | 44.95 | 40.78 | 47.05 | +2.0% | -8.7% .. +9.6% | within noise |
|  | `configure-notify` | 47.98 | 42.88 | 60.51 | +8.8% | -3.0% .. +17.4% | within noise |
|  | `fullscreen-state` | 42.72 | 38.51 | 52.22 | -3.1% | -14.6% .. +8.5% | within noise |
| focus_m2 | `main` | 48.30 | 34.35 | 61.10 |  |  | A/A floor ±20.6% |
|  | `main-again` | 48.03 | 35.92 | 56.96 | -0.6% | -21.0% .. +22.0% | A/A |
|  | `stable-primary` | 43.09 | 35.80 | 56.14 | -10.8% | -23.2% .. +12.5% | within noise |
|  | `stable-primary-17c0827` | 41.51 | 36.17 | 50.28 | -14.1% | -22.7% .. +11.3% | within noise |
|  | `configure-notify` | 42.83 | 36.37 | 54.56 | -11.3% | -23.7% .. +11.7% | within noise |
|  | `fullscreen-state` | 39.31 | 35.28 | 54.95 | -18.6% | -27.3% .. +9.6% | within noise |
| focus_m200 | `main` | 38.98 | 35.58 | 50.24 |  |  | A/A floor ±26.4% |
|  | `main-again` | 41.63 | 36.15 | 53.08 | +6.8% | -9.9% .. +26.4% | A/A |
|  | `stable-primary` | 43.75 | 36.92 | 53.21 | +12.2% | -8.7% .. +25.4% | within noise |
|  | `stable-primary-17c0827` | 43.44 | 37.17 | 59.80 | +11.4% | -8.4% .. +30.7% | within noise |
|  | `configure-notify` | 39.78 | 35.91 | 59.42 | +2.1% | -14.8% .. +24.4% | within noise |
|  | `fullscreen-state` | 41.16 | 36.15 | 55.95 | +5.6% | -15.2% .. +23.3% | within noise |
| focus_m3_u1000 | `main` | 44.92 | 35.49 | 51.83 |  |  | A/A floor ±19.4% |
|  | `main-again` | 43.88 | 35.60 | 49.83 | -2.3% | -19.3% .. +16.5% | A/A |
|  | `stable-primary` | 38.76 | 36.33 | 51.10 | -13.7% | -20.7% .. +12.5% | within noise |
|  | `stable-primary-17c0827` | 40.17 | 37.26 | 56.15 | -10.6% | -18.1% .. +12.2% | within noise |
|  | `configure-notify` | 39.65 | 35.18 | 51.70 | -11.7% | -22.5% .. +7.3% | within noise |
|  | `fullscreen-state` | 44.02 | 35.52 | 102.81 | -2.0% | -17.3% .. +18.8% | within noise |
| focus_m50 | `main` | 43.11 | 35.58 | 124.98 |  |  | A/A floor ±23.7% |
|  | `main-again` | 45.55 | 35.43 | 54.97 | +5.7% | -18.8% .. +24.3% | A/A |
|  | `stable-primary` | 39.35 | 36.38 | 50.45 | -8.7% | -25.8% .. +22.5% | within noise |
|  | `stable-primary-17c0827` | 49.53 | 37.48 | 61.25 | +14.9% | -9.2% .. +36.6% | within noise |
|  | `configure-notify` | 46.87 | 36.02 | 59.62 | +8.7% | -16.7% .. +23.9% | within noise |
|  | `fullscreen-state` | 40.57 | 35.57 | 139.13 | -5.9% | -23.2% .. +17.9% | within noise |
| fullscreen_redundant | `main` | 102.10 | 88.48 | 179.94 |  |  | A/A floor ±13.8% |
|  | `main-again` | 98.57 | 89.08 | 115.44 | -3.5% | -13.7% .. +7.2% | A/A |
|  | `stable-primary` | 100.80 | 88.67 | 109.40 | -1.3% | -12.3% .. +8.3% | within noise |
|  | `stable-primary-17c0827` | 107.28 | 95.19 | 139.59 | +5.1% | -6.8% .. +16.6% | within noise |
|  | `configure-notify` | 104.44 | 89.47 | 152.17 | +2.3% | -10.4% .. +17.0% | within noise |
|  | `fullscreen-state` | 113.01 | 103.54 | 320.72 | +10.7% | -0.4% .. +24.5% | within noise |
| fullscreen_toggle | `main` | 25.97 | 22.24 | 33.64 |  |  | A/A floor ±15.4% |
|  | `main-again` | 27.36 | 25.68 | 31.90 | +5.4% | -3.1% .. +14.9% | A/A |
|  | `stable-primary` | 26.75 | 25.28 | 29.10 | +3.0% | -5.0% .. +9.3% | within noise |
|  | `stable-primary-17c0827` | 27.48 | 24.18 | 30.67 | +5.8% | -2.8% .. +13.9% | within noise |
|  | `configure-notify` | 27.54 | 23.32 | 32.56 | +6.0% | -4.4% .. +17.0% | within noise |
|  | `fullscreen-state` | 36.14 | 33.24 | 49.85 | +39.2% | +27.4% .. +54.9% | slower |
| fullscreen_toggle_wait | `main` | 39.80 | 36.89 | 55.31 |  |  | A/A floor ±13.4% |
|  | `main-again` | 39.63 | 36.94 | 48.75 | -0.4% | -9.1% .. +13.4% | A/A |
|  | `stable-primary` | 38.99 | 37.03 | 48.04 | -2.0% | -10.7% .. +4.1% | within noise |
|  | `stable-primary-17c0827` | 39.69 | 37.34 | 48.28 | -0.3% | -8.9% .. +6.5% | within noise |
|  | `configure-notify` | 38.95 | 37.31 | 41.60 | -2.2% | -11.1% .. +4.9% | within noise |
|  | `fullscreen-state` | 51.20 | 47.00 | 94.75 | +28.6% | +15.6% .. +47.8% | slower |
| map_churn | `main` | 302.53 | 282.82 | 351.64 |  |  | A/A floor ±11.8% |
|  | `main-again` | 308.45 | 279.04 | 388.27 | +2.0% | -6.5% .. +10.9% | A/A |
|  | `stable-primary` | 296.74 | 288.37 | 384.79 | -1.9% | -7.2% .. +12.9% | within noise |
|  | `stable-primary-17c0827` | 308.94 | 292.61 | 350.16 | +2.1% | -4.7% .. +13.3% | within noise |
|  | `configure-notify` | 306.04 | 279.19 | 449.36 | +1.2% | -6.9% .. +13.5% | within noise |
|  | `fullscreen-state` | 309.92 | 279.84 | 416.27 | +2.4% | -7.6% .. +16.9% | within noise |
| output_enter_m200 | `main` | 71.94 | 57.77 | 93.28 |  |  | A/A floor ±25.3% |
|  | `main-again` | 64.36 | 57.33 | 90.67 | -10.5% | -24.9% .. +15.3% | A/A |
|  | `stable-primary` | 20.20 | 16.52 | 23.84 | -71.9% | -77.9% .. -67.7% | faster |
|  | `stable-primary-17c0827` | 18.88 | 16.66 | 25.14 | -73.8% | -78.6% .. -68.5% | faster |
|  | `configure-notify` | 68.80 | 57.37 | 87.97 | -4.4% | -21.1% .. +18.9% | within noise |
|  | `fullscreen-state` | 61.61 | 57.28 | 101.73 | -14.4% | -27.8% .. +1.8% | within noise |
| output_enter_m3 | `main` | 76.72 | 55.46 | 116.79 |  |  | A/A floor ±25.4% |
|  | `main-again` | 65.26 | 55.20 | 82.62 | -14.9% | -25.7% .. +20.5% | A/A |
|  | `stable-primary` | 20.02 | 16.16 | 21.81 | -73.9% | -78.8% .. -66.1% | faster |
|  | `stable-primary-17c0827` | 21.27 | 16.13 | 30.57 | -72.3% | -76.3% .. -64.2% | faster |
|  | `configure-notify` | 73.44 | 55.01 | 88.89 | -4.3% | -21.9% .. +28.2% | within noise |
|  | `fullscreen-state` | 65.08 | 56.26 | 96.41 | -15.2% | -25.5% .. +19.9% | within noise |
| output_enter_m3_u1000 | `main` | 100.75 | 75.38 | 106.33 |  |  | A/A floor ±20.4% |
|  | `main-again` | 94.81 | 74.86 | 99.72 | -5.9% | -20.1% .. +2.3% | A/A |
|  | `stable-primary` | 31.75 | 25.58 | 37.32 | -68.5% | -71.7% .. -65.9% | faster |
|  | `stable-primary-17c0827` | 31.52 | 26.13 | 40.85 | -68.7% | -72.9% .. -65.4% | faster |
|  | `configure-notify` | 93.69 | 73.77 | 118.03 | -7.0% | -17.9% .. +2.7% | within noise |
|  | `fullscreen-state` | 94.16 | 73.98 | 171.63 | -6.5% | -23.9% .. +5.4% | within noise |

## satellite CPU per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| barrier_only | `main` | 42.74 | 38.25 | 53.34 |  |  | A/A floor ±11.3% |
|  | `main-again` | 42.52 | 38.43 | 47.74 | -0.5% | -8.7% .. +11.8% | A/A |
|  | `stable-primary` | 40.16 | 38.42 | 47.71 | -6.0% | -10.7% .. +8.7% | within noise |
|  | `stable-primary-17c0827` | 43.36 | 38.71 | 49.32 | +1.4% | -7.0% .. +12.7% | within noise |
|  | `configure-notify` | 44.99 | 38.60 | 65.91 | +5.3% | -1.8% .. +15.2% | within noise |
|  | `fullscreen-state` | 45.08 | 38.05 | 47.41 | +5.5% | -2.2% .. +15.1% | within noise |
| configure_position | `main` | 7.00 | 6.80 | 8.74 |  |  | A/A floor ±4.7% |
|  | `main-again` | 7.08 | 6.88 | 7.48 | +1.2% | -4.4% .. +4.7% | A/A |
|  | `stable-primary` | 7.05 | 6.82 | 7.55 | +0.6% | -4.5% .. +3.1% | within noise |
|  | `stable-primary-17c0827` | 7.02 | 6.79 | 7.27 | +0.3% | -4.6% .. +3.4% | within noise |
|  | `configure-notify` | 9.30 | 7.16 | 10.10 | +32.9% | +7.4% .. +35.8% | slower |
|  | `fullscreen-state` | 7.00 | 6.86 | 7.44 | +0.0% | -4.4% .. +3.3% | within noise |
| configure_position_size | `main` | 23.11 | 20.79 | 35.99 |  |  | A/A floor ±7.7% |
|  | `main-again` | 23.13 | 20.82 | 24.75 | +0.1% | -7.6% .. +7.3% | A/A |
|  | `stable-primary` | 22.22 | 20.95 | 25.55 | -3.9% | -10.0% .. +4.5% | within noise |
|  | `stable-primary-17c0827` | 23.65 | 21.07 | 24.73 | +2.3% | -7.4% .. +9.0% | within noise |
|  | `configure-notify` | 26.54 | 23.59 | 32.07 | +14.8% | +7.6% .. +23.5% | within noise |
|  | `fullscreen-state` | 22.10 | 20.69 | 25.57 | -4.4% | -10.9% .. +6.1% | within noise |
| configure_position_sync | `main` | 22.45 | 20.20 | 31.85 |  |  | A/A floor ±7.4% |
|  | `main-again` | 21.85 | 20.78 | 23.97 | -2.7% | -7.4% .. +3.8% | A/A |
|  | `stable-primary` | 21.35 | 20.57 | 24.04 | -4.9% | -9.9% .. +2.3% | within noise |
|  | `stable-primary-17c0827` | 22.18 | 20.72 | 23.17 | -1.2% | -5.5% .. +4.9% | within noise |
|  | `configure-notify` | 23.02 | 21.23 | 28.09 | +2.5% | -2.5% .. +9.2% | within noise |
|  | `fullscreen-state` | 21.66 | 20.35 | 25.25 | -3.5% | -9.6% .. +5.6% | within noise |
| focus_m2 | `main` | 24.27 | 19.43 | 30.26 |  |  | A/A floor ±18.3% |
|  | `main-again` | 24.57 | 19.80 | 27.62 | +1.2% | -15.5% .. +15.4% | A/A |
|  | `stable-primary` | 22.40 | 20.13 | 27.17 | -7.7% | -16.7% .. +13.2% | within noise |
|  | `stable-primary-17c0827` | 22.30 | 20.27 | 26.16 | -8.1% | -15.7% .. +11.4% | within noise |
|  | `configure-notify` | 22.11 | 20.08 | 26.54 | -8.9% | -18.0% .. +8.1% | within noise |
|  | `fullscreen-state` | 20.80 | 19.61 | 26.89 | -14.3% | -20.5% .. +6.9% | within noise |
| focus_m200 | `main` | 21.09 | 19.83 | 25.27 |  |  | A/A floor ±18.2% |
|  | `main-again` | 21.89 | 20.04 | 25.75 | +3.8% | -5.7% .. +18.2% | A/A |
|  | `stable-primary` | 23.12 | 20.77 | 26.45 | +9.7% | -3.3% .. +22.2% | within noise |
|  | `stable-primary-17c0827` | 23.26 | 20.87 | 28.90 | +10.3% | -2.1% .. +25.3% | within noise |
|  | `configure-notify` | 20.90 | 19.99 | 29.68 | -0.9% | -9.3% .. +13.6% | within noise |
|  | `fullscreen-state` | 21.55 | 19.94 | 27.03 | +2.2% | -7.8% .. +16.3% | within noise |
| focus_m3_u1000 | `main` | 23.05 | 19.63 | 25.37 |  |  | A/A floor ±13.8% |
|  | `main-again` | 22.11 | 19.80 | 24.64 | -4.1% | -14.9% .. +11.3% | A/A |
|  | `stable-primary` | 21.45 | 20.63 | 26.50 | -7.0% | -12.6% .. +16.1% | within noise |
|  | `stable-primary-17c0827` | 22.06 | 21.05 | 28.44 | -4.3% | -10.9% .. +13.6% | within noise |
|  | `configure-notify` | 21.16 | 19.64 | 24.85 | -8.2% | -17.1% .. +7.4% | within noise |
|  | `fullscreen-state` | 23.30 | 19.72 | 35.19 | +1.1% | -12.0% .. +18.9% | within noise |
| focus_m50 | `main` | 22.53 | 19.78 | 37.71 |  |  | A/A floor ±18.7% |
|  | `main-again` | 22.59 | 20.01 | 28.84 | +0.3% | -14.7% .. +18.4% | A/A |
|  | `stable-primary` | 21.18 | 20.29 | 25.73 | -6.0% | -17.5% .. +20.4% | within noise |
|  | `stable-primary-17c0827` | 25.39 | 20.87 | 30.65 | +12.7% | -7.3% .. +30.5% | within noise |
|  | `configure-notify` | 23.67 | 19.88 | 31.31 | +5.1% | -13.6% .. +17.6% | within noise |
|  | `fullscreen-state` | 21.40 | 19.84 | 38.72 | -5.0% | -17.4% .. +14.6% | within noise |
| fullscreen_redundant | `main` | 56.99 | 49.67 | 90.57 |  |  | A/A floor ±12.9% |
|  | `main-again` | 55.09 | 49.95 | 61.32 | -3.3% | -13.2% .. +6.6% | A/A |
|  | `stable-primary` | 56.44 | 49.94 | 60.25 | -1.0% | -10.2% .. +9.8% | within noise |
|  | `stable-primary-17c0827` | 59.09 | 51.87 | 75.76 | +3.7% | -6.8% .. +14.9% | within noise |
|  | `configure-notify` | 58.25 | 49.76 | 73.25 | +2.2% | -11.1% .. +13.6% | within noise |
|  | `fullscreen-state` | 61.92 | 57.64 | 107.33 | +8.7% | -0.4% .. +23.2% | within noise |
| fullscreen_toggle | `main` | 15.13 | 13.07 | 18.15 |  |  | A/A floor ±12.6% |
|  | `main-again` | 16.17 | 14.93 | 17.37 | +6.9% | -2.1% .. +12.7% | A/A |
|  | `stable-primary` | 15.98 | 14.98 | 16.98 | +5.6% | -3.4% .. +10.4% | within noise |
|  | `stable-primary-17c0827` | 16.32 | 14.57 | 17.59 | +7.8% | -1.0% .. +12.4% | within noise |
|  | `configure-notify` | 16.05 | 13.84 | 17.96 | +6.1% | -2.4% .. +13.5% | within noise |
|  | `fullscreen-state` | 20.70 | 19.24 | 24.80 | +36.8% | +25.9% .. +45.7% | slower |
| fullscreen_toggle_wait | `main` | 27.30 | 25.90 | 34.96 |  |  | A/A floor ±5.7% |
|  | `main-again` | 27.43 | 26.01 | 32.31 | +0.5% | -4.4% .. +5.9% | A/A |
|  | `stable-primary` | 27.06 | 25.88 | 30.73 | -0.9% | -5.7% .. +3.3% | within noise |
|  | `stable-primary-17c0827` | 27.32 | 26.39 | 30.28 | +0.1% | -4.9% .. +4.3% | within noise |
|  | `configure-notify` | 27.00 | 26.41 | 28.06 | -1.1% | -5.4% .. +2.4% | within noise |
|  | `fullscreen-state` | 32.68 | 31.22 | 46.82 | +19.7% | +13.9% .. +29.6% | slower |
| map_churn | `main` | 164.71 | 157.13 | 192.99 |  |  | A/A floor ±6.6% |
|  | `main-again` | 165.87 | 156.13 | 199.54 | +0.7% | -5.3% .. +6.1% | A/A |
|  | `stable-primary` | 165.72 | 161.92 | 191.38 | +0.6% | -5.1% .. +8.3% | within noise |
|  | `stable-primary-17c0827` | 168.41 | 163.73 | 193.01 | +2.2% | -3.2% .. +9.7% | within noise |
|  | `configure-notify` | 166.46 | 157.84 | 213.70 | +1.1% | -5.7% .. +8.7% | within noise |
|  | `fullscreen-state` | 165.40 | 156.79 | 197.18 | +0.4% | -6.4% .. +10.5% | within noise |
| output_enter_m200 | `main` | 32.76 | 29.03 | 43.72 |  |  | A/A floor ±21.8% |
|  | `main-again` | 31.16 | 28.97 | 40.26 | -4.9% | -22.2% .. +18.7% | A/A |
|  | `stable-primary` | 10.68 | 9.52 | 12.50 | -67.4% | -73.6% .. -62.8% | faster |
|  | `stable-primary-17c0827` | 10.73 | 9.67 | 14.27 | -67.2% | -74.1% .. -63.0% | faster |
|  | `configure-notify` | 33.08 | 28.92 | 39.48 | +1.0% | -19.3% .. +18.1% | within noise |
|  | `fullscreen-state` | 30.16 | 29.05 | 45.81 | -7.9% | -23.7% .. +2.1% | within noise |
| output_enter_m3 | `main` | 37.10 | 28.81 | 47.66 |  |  | A/A floor ±21.7% |
|  | `main-again` | 32.07 | 29.19 | 39.73 | -13.6% | -21.7% .. +16.4% | A/A |
|  | `stable-primary` | 10.87 | 9.35 | 11.72 | -70.7% | -74.7% .. -63.6% | faster |
|  | `stable-primary-17c0827` | 11.73 | 9.32 | 14.68 | -68.4% | -72.9% .. -61.0% | faster |
|  | `configure-notify` | 35.25 | 28.80 | 45.37 | -5.0% | -19.3% .. +23.6% | within noise |
|  | `fullscreen-state` | 32.46 | 29.22 | 46.55 | -12.5% | -21.0% .. +18.9% | within noise |
| output_enter_m3_u1000 | `main` | 41.58 | 32.27 | 44.19 |  |  | A/A floor ±19.3% |
|  | `main-again` | 36.27 | 32.21 | 42.57 | -12.8% | -18.8% .. +0.7% | A/A |
|  | `stable-primary` | 15.19 | 12.72 | 16.98 | -63.5% | -67.2% .. -61.9% | faster |
|  | `stable-primary-17c0827` | 15.04 | 12.69 | 18.68 | -63.8% | -68.6% .. -61.6% | faster |
|  | `configure-notify` | 37.60 | 32.21 | 47.09 | -9.6% | -17.4% .. +2.1% | within noise |
|  | `fullscreen-state` | 41.80 | 32.06 | 57.75 | +0.5% | -20.6% .. +3.8% | within noise |

## Xwayland CPU per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| barrier_only | `main` | 47.60 | 43.64 | 64.09 |  |  | A/A floor ±7.1% |
|  | `main-again` | 48.61 | 44.22 | 55.91 | +2.1% | -5.2% .. +7.3% | A/A |
|  | `stable-primary` | 46.60 | 44.39 | 58.76 | -2.1% | -8.4% .. +5.8% | within noise |
|  | `stable-primary-17c0827` | 49.86 | 44.53 | 55.50 | +4.7% | -7.1% .. +8.5% | within noise |
|  | `configure-notify` | 49.48 | 44.19 | 79.01 | +4.0% | -4.2% .. +9.3% | within noise |
|  | `fullscreen-state` | 50.50 | 43.01 | 53.81 | +6.1% | -3.7% .. +12.8% | within noise |
| configure_position | `main` | 6.75 | 6.58 | 8.55 |  |  | A/A floor ±4.1% |
|  | `main-again` | 6.83 | 6.63 | 7.22 | +1.1% | -3.8% .. +3.8% | A/A |
|  | `stable-primary` | 6.82 | 6.60 | 7.38 | +1.0% | -3.4% .. +2.8% | within noise |
|  | `stable-primary-17c0827` | 6.80 | 6.56 | 7.06 | +0.7% | -3.9% .. +3.2% | within noise |
|  | `configure-notify` | 10.76 | 7.93 | 11.21 | +59.2% | +23.0% .. +63.6% | slower |
|  | `fullscreen-state` | 6.76 | 6.59 | 7.16 | +0.0% | -4.3% .. +2.6% | within noise |
| configure_position_size | `main` | 52.36 | 47.43 | 85.22 |  |  | A/A floor ±7.0% |
|  | `main-again` | 52.20 | 47.37 | 56.76 | -0.3% | -6.9% .. +6.8% | A/A |
|  | `stable-primary` | 51.16 | 47.41 | 57.70 | -2.3% | -9.1% .. +5.4% | within noise |
|  | `stable-primary-17c0827` | 53.42 | 48.43 | 57.15 | +2.0% | -6.0% .. +8.3% | within noise |
|  | `configure-notify` | 59.66 | 52.63 | 72.55 | +13.9% | +6.0% .. +21.6% | within noise |
|  | `fullscreen-state` | 50.72 | 47.05 | 62.47 | -3.1% | -9.7% .. +6.2% | within noise |
| configure_position_sync | `main` | 22.91 | 20.30 | 32.16 |  |  | A/A floor ±7.8% |
|  | `main-again` | 22.10 | 20.94 | 24.12 | -3.6% | -7.6% .. +6.4% | A/A |
|  | `stable-primary` | 21.50 | 20.82 | 23.93 | -6.2% | -9.6% .. +3.4% | within noise |
|  | `stable-primary-17c0827` | 23.02 | 21.19 | 23.58 | +0.4% | -3.9% .. +8.8% | within noise |
|  | `configure-notify` | 25.17 | 22.61 | 28.55 | +9.8% | +5.1% .. +19.0% | within noise |
|  | `fullscreen-state` | 22.10 | 20.70 | 24.93 | -3.6% | -9.5% .. +6.6% | within noise |
| focus_m2 | `main` | 27.43 | 21.20 | 32.95 |  |  | A/A floor ±19.0% |
|  | `main-again` | 27.75 | 21.75 | 29.51 | +1.1% | -17.7% .. +19.2% | A/A |
|  | `stable-primary` | 23.91 | 21.85 | 29.93 | -12.9% | -20.9% .. +14.5% | within noise |
|  | `stable-primary-17c0827` | 23.65 | 21.77 | 28.34 | -13.8% | -20.1% .. +7.5% | within noise |
|  | `configure-notify` | 23.74 | 22.05 | 29.40 | -13.5% | -20.4% .. +9.0% | within noise |
|  | `fullscreen-state` | 22.56 | 21.67 | 30.06 | -17.7% | -23.1% .. +6.8% | within noise |
| focus_m200 | `main` | 23.32 | 21.71 | 29.11 |  |  | A/A floor ±22.2% |
|  | `main-again` | 24.21 | 22.05 | 28.83 | +3.8% | -4.6% .. +22.2% | A/A |
|  | `stable-primary` | 24.54 | 22.33 | 28.33 | +5.2% | -5.1% .. +20.4% | within noise |
|  | `stable-primary-17c0827` | 24.28 | 22.36 | 31.99 | +4.1% | -4.4% .. +23.7% | within noise |
|  | `configure-notify` | 23.10 | 21.97 | 30.98 | -0.9% | -7.8% .. +16.1% | within noise |
|  | `fullscreen-state` | 23.59 | 22.06 | 29.60 | +1.2% | -8.0% .. +18.1% | within noise |
| focus_m3_u1000 | `main` | 25.51 | 21.76 | 28.70 |  |  | A/A floor ±17.0% |
|  | `main-again` | 24.00 | 21.83 | 27.93 | -5.9% | -17.0% .. +13.4% | A/A |
|  | `stable-primary` | 22.57 | 21.76 | 28.65 | -11.5% | -18.9% .. +9.2% | within noise |
|  | `stable-primary-17c0827` | 22.88 | 22.03 | 30.43 | -10.3% | -17.7% .. +9.0% | within noise |
|  | `configure-notify` | 22.80 | 21.69 | 27.91 | -10.6% | -18.7% .. +7.2% | within noise |
|  | `fullscreen-state` | 24.77 | 21.78 | 38.20 | -2.9% | -15.5% .. +21.3% | within noise |
| focus_m50 | `main` | 24.49 | 21.80 | 39.87 |  |  | A/A floor ±21.4% |
|  | `main-again` | 24.32 | 21.94 | 30.97 | -0.7% | -16.6% .. +21.3% | A/A |
|  | `stable-primary` | 22.61 | 21.72 | 28.29 | -7.6% | -22.1% .. +19.4% | within noise |
|  | `stable-primary-17c0827` | 28.16 | 22.20 | 30.69 | +15.0% | -12.4% .. +29.0% | within noise |
|  | `configure-notify` | 26.75 | 21.94 | 32.71 | +9.2% | -14.9% .. +21.0% | within noise |
|  | `fullscreen-state` | 23.13 | 21.57 | 41.75 | -5.5% | -19.2% .. +15.9% | within noise |
| fullscreen_redundant | `main` | 62.88 | 57.50 | 100.18 |  |  | A/A floor ±10.0% |
|  | `main-again` | 61.81 | 57.60 | 69.11 | -1.7% | -11.0% .. +5.6% | A/A |
|  | `stable-primary` | 62.77 | 57.27 | 67.15 | -0.2% | -9.3% .. +5.3% | within noise |
|  | `stable-primary-17c0827` | 65.74 | 60.07 | 82.04 | +4.6% | -4.8% .. +11.1% | within noise |
|  | `configure-notify` | 64.47 | 57.22 | 81.31 | +2.5% | -7.3% .. +11.4% | within noise |
|  | `fullscreen-state` | 71.64 | 67.83 | 118.04 | +13.9% | +5.0% .. +24.2% | within noise |
| fullscreen_toggle | `main` | 13.97 | 12.67 | 17.15 |  |  | A/A floor ±10.7% |
|  | `main-again` | 14.84 | 13.91 | 16.33 | +6.2% | -1.5% .. +10.8% | A/A |
|  | `stable-primary` | 14.65 | 14.01 | 15.72 | +4.9% | -3.3% .. +7.6% | within noise |
|  | `stable-primary-17c0827` | 15.00 | 13.62 | 16.66 | +7.3% | -0.8% .. +11.3% | within noise |
|  | `configure-notify` | 14.81 | 13.44 | 16.62 | +6.0% | -2.9% .. +10.1% | within noise |
|  | `fullscreen-state` | 20.62 | 19.69 | 25.03 | +47.6% | +37.1% .. +52.9% | slower |
| fullscreen_toggle_wait | `main` | 18.98 | 18.04 | 24.32 |  |  | A/A floor ±5.4% |
|  | `main-again` | 18.96 | 18.25 | 22.16 | -0.1% | -4.5% .. +5.6% | A/A |
|  | `stable-primary` | 18.70 | 18.00 | 21.29 | -1.4% | -5.7% .. +2.9% | within noise |
|  | `stable-primary-17c0827` | 19.05 | 18.37 | 21.11 | +0.4% | -4.4% .. +4.7% | within noise |
|  | `configure-notify` | 18.77 | 18.24 | 19.53 | -1.1% | -5.6% .. +2.7% | within noise |
|  | `fullscreen-state` | 25.50 | 24.34 | 35.18 | +34.4% | +28.0% .. +43.7% | slower |
| map_churn | `main` | 182.75 | 176.16 | 217.20 |  |  | A/A floor ±6.3% |
|  | `main-again` | 184.62 | 175.26 | 226.87 | +1.0% | -5.3% .. +5.9% | A/A |
|  | `stable-primary` | 184.58 | 181.03 | 212.79 | +1.0% | -5.5% .. +8.4% | within noise |
|  | `stable-primary-17c0827` | 189.49 | 182.30 | 215.06 | +3.7% | -2.7% .. +10.3% | within noise |
|  | `configure-notify` | 187.74 | 174.84 | 269.84 | +2.7% | -5.5% .. +8.5% | within noise |
|  | `fullscreen-state` | 186.65 | 175.51 | 228.45 | +2.1% | -5.6% .. +10.5% | within noise |
| output_enter_m200 | `main` | 41.87 | 37.93 | 53.46 |  |  | A/A floor ±21.1% |
|  | `main-again` | 40.23 | 37.42 | 51.26 | -3.9% | -21.2% .. +18.7% | A/A |
|  | `stable-primary` | 11.65 | 10.40 | 14.35 | -72.2% | -77.3% .. -67.2% | faster |
|  | `stable-primary-17c0827` | 11.35 | 10.49 | 14.93 | -72.9% | -78.1% .. -67.8% | faster |
|  | `configure-notify` | 42.12 | 37.56 | 51.59 | +0.6% | -18.6% .. +19.5% | within noise |
|  | `fullscreen-state` | 39.10 | 37.68 | 60.37 | -6.6% | -23.1% .. +1.6% | within noise |
| output_enter_m3 | `main` | 46.08 | 35.21 | 55.62 |  |  | A/A floor ±22.1% |
|  | `main-again` | 38.81 | 35.48 | 47.95 | -15.8% | -22.2% .. +18.0% | A/A |
|  | `stable-primary` | 10.37 | 9.19 | 12.47 | -77.5% | -79.9% .. -68.9% | faster |
|  | `stable-primary-17c0827` | 12.24 | 9.08 | 13.86 | -73.4% | -78.0% .. -66.5% | faster |
|  | `configure-notify` | 42.56 | 35.06 | 54.77 | -7.6% | -20.2% .. +23.1% | within noise |
|  | `fullscreen-state` | 38.67 | 35.30 | 56.63 | -16.1% | -21.6% .. +19.8% | within noise |
| output_enter_m3_u1000 | `main` | 64.09 | 51.95 | 66.57 |  |  | A/A floor ±15.7% |
|  | `main-again` | 57.91 | 51.96 | 65.95 | -9.6% | -15.7% .. +1.4% | A/A |
|  | `stable-primary` | 20.79 | 16.75 | 23.08 | -67.6% | -70.8% .. -65.7% | faster |
|  | `stable-primary-17c0827` | 20.73 | 17.30 | 24.89 | -67.6% | -72.0% .. -65.5% | faster |
|  | `configure-notify` | 60.03 | 51.26 | 71.79 | -6.3% | -13.6% .. +1.2% | within noise |
|  | `fullscreen-state` | 61.58 | 51.73 | 93.55 | -3.9% | -17.4% .. +3.0% | within noise |

## Full `cargo test --release` suite (s)

| tree | runs | median | p10-p90 | vs baseline | failed runs |
|---|---|---|---|---|---|
| `main` | 5 | 10.46 | 10.19-10.71 | +0.0% | 0 |
| `main-again` | 5 | 10.54 | 10.08-10.83 | +0.7% | 0 |
| `stable-primary` | 5 | 11.39 | 10.97-11.46 | +8.9% | 0 |
| `stable-primary-17c0827` | 5 | 11.27 | 11.08-11.41 | +7.7% | 0 |
| `configure-notify` | 5 | 10.63 | 10.39-10.75 | +1.6% | 0 |
| `fullscreen-state` | 5 | 10.48 | 10.37-10.85 | +0.2% | 0 |

