# Benchmark results

Trees:

- `main`: b424e5e (b424e5e44f), 20 rounds
- `main-again`: b424e5e (b424e5e44f), 20 rounds
- `stable-primary`: 17c0827 (17c08273be), 20 rounds
- `stable-primary-1st`: d9cd02b (d9cd02bf14), 20 rounds
- `configure-notify`: c45fab6 (c45fab6e4c), 20 rounds
- `fullscreen-state`: fb0ed3b (fb0ed3b7f0), 20 rounds

Baseline `main`. A/A control `main-again` (same commit) sets the noise floor. Each sample is one process's median over its runs; tables give the median, p10 and p90 over rounds and the change of the median vs the baseline with a bootstrap 95% CI. A change is marked slower/faster only when its whole CI lies beyond the A/A noise floor and beyond 2%.

## Overview: change of the median vs baseline (wall / satellite CPU)

| benchmark | ops | `main` wall, sat CPU (us) | `main-again` | `stable-primary` | `stable-primary-1st` | `configure-notify` | `fullscreen-state` |
|---|---|---|---|---|---|---|---|
| barrier_only | 500 | 83.2, 45.3 | -3.6% / -4.4% | -8.2% / -10.9% | -4.8% / -4.7% | +0.5% / +0.3% | -0.6% / -4.7% |
| configure_position | 5000 | 12.4, 7.0 | -1.6% / +0.3% | -0.2% / -0.1% | -1.5% / -0.7% | +250.8% **slower** / +203.4% **slower** | -0.8% / -0.3% |
| configure_position_size | 1000 | 71.0, 23.8 | -4.8% / -1.0% | -4.3% / -2.4% | -7.0% / -2.4% | - / - | -5.0% / -1.9% |
| configure_position_sync | 1000 | 45.3, 22.2 | +1.3% / +0.2% | -2.0% / -1.7% | +2.3% / +0.8% | +47.0% **slower** / +44.1% **slower** | -1.1% / -0.3% |
| focus_m2 | 2000 | 41.8, 21.2 | -6.9% / -2.7% | +8.5% / +8.1% | +5.1% / +6.3% | +19.8% / +16.0% | +8.8% / +11.4% |
| focus_m200 | 2000 | 41.1, 21.6 | +1.5% / +0.5% | +2.4% / +4.5% | -4.5% / -3.4% | +18.0% / +11.5% | +5.9% / +4.4% |
| focus_m3_u1000 | 2000 | 38.1, 20.6 | +19.0% / +12.2% | +13.5% / +9.7% | +4.5% / +1.7% | +14.0% / +3.4% | +22.6% / +12.1% |
| focus_m50 | 2000 | 42.1, 21.5 | +5.7% / +5.7% | +5.6% / +8.1% | +3.6% / +2.2% | +8.3% / +4.6% | -2.7% / -1.2% |
| fullscreen_redundant | 500 | 102.8, 56.8 | -2.6% / -5.9% | -1.2% / -1.0% | +1.8% / +3.0% | +1.7% / -0.6% | +13.6% / +11.5% |
| fullscreen_toggle | 2000 | 27.1, 15.8 | +2.7% / +2.4% | +1.5% / +1.7% | +1.9% / +2.0% | +4.9% / +1.5% | +41.4% **slower** / +35.7% **slower** |
| fullscreen_toggle_wait | 500 | 40.3, 27.0 | +1.9% / +1.0% | +2.5% / +1.6% | +1.3% / +2.0% | +1.4% / +1.2% | +28.0% **slower** / +21.5% **slower** |
| map_churn | 200 | 303.9, 165.8 | +2.8% / +0.5% | +7.1% / +3.9% | +3.7% / +0.5% | +4.0% / +1.5% | +1.4% / -0.4% |
| output_enter_m200 | 2000 | 69.4, 32.4 | -2.0% / -3.2% | -74.1% **faster** / -68.8% **faster** | -71.5% **faster** / -69.3% **faster** | -5.2% / -4.6% | -4.9% / -4.2% |
| output_enter_m3 | 2000 | 67.9, 32.1 | +3.0% / +1.7% | -69.8% **faster** / -66.9% **faster** | -70.7% **faster** / -67.9% **faster** | -5.0% / -2.7% | +17.0% / +19.7% |
| output_enter_m3_u1000 | 2000 | 98.6, 39.7 | -5.5% / -7.4% | -69.9% **faster** / -64.9% **faster** | -71.8% **faster** / -68.8% **faster** | -9.6% / -8.9% | -17.1% / -15.1% |

## wall time per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| barrier_only | `main` | 83.25 | 71.50 | 142.26 |  |  | A/A floor ±12.9% |
|  | `main-again` | 80.26 | 69.54 | 87.67 | -3.6% | -12.5% .. +10.3% | A/A |
|  | `stable-primary` | 76.38 | 70.27 | 87.46 | -8.2% | -15.1% .. +7.0% | within noise |
|  | `stable-primary-1st` | 79.24 | 72.81 | 94.31 | -4.8% | -12.1% .. +9.1% | within noise |
|  | `configure-notify` | 83.70 | 72.44 | 90.68 | +0.5% | -9.5% .. +14.5% | within noise |
|  | `fullscreen-state` | 82.77 | 72.50 | 91.07 | -0.6% | -12.5% .. +13.2% | within noise |
| configure_position | `main` | 12.40 | 11.63 | 15.87 |  |  | A/A floor ±6.1% |
|  | `main-again` | 12.20 | 11.91 | 13.23 | -1.6% | -6.1% .. +3.5% | A/A |
|  | `stable-primary` | 12.38 | 11.98 | 13.12 | -0.2% | -4.7% .. +4.1% | within noise |
|  | `stable-primary-1st` | 12.21 | 11.84 | 13.87 | -1.5% | -6.6% .. +2.7% | within noise |
|  | `configure-notify` | 43.48 | 35.29 | 48.36 | +250.8% | +205.1% .. +280.3% | slower |
|  | `fullscreen-state` | 12.29 | 11.70 | 13.19 | -0.8% | -5.9% .. +3.6% | within noise |
| configure_position_size | `main` | 70.96 | 60.60 | 88.41 |  |  | A/A floor ±11.0% |
|  | `main-again` | 67.58 | 64.05 | 77.17 | -4.8% | -10.8% .. +3.3% | A/A |
|  | `stable-primary` | 67.92 | 63.26 | 72.16 | -4.3% | -10.4% .. +1.3% | within noise |
|  | `stable-primary-1st` | 66.00 | 60.65 | 75.70 | -7.0% | -13.2% .. +2.2% | within noise |
|  | `fullscreen-state` | 67.43 | 63.11 | 75.62 | -5.0% | -10.9% .. +4.5% | within noise |
| configure_position_sync | `main` | 45.35 | 40.91 | 52.23 |  |  | A/A floor ±10.5% |
|  | `main-again` | 45.94 | 42.08 | 51.19 | +1.3% | -7.5% .. +10.7% | A/A |
|  | `stable-primary` | 44.45 | 40.51 | 46.91 | -2.0% | -8.8% .. +6.8% | within noise |
|  | `stable-primary-1st` | 46.39 | 41.76 | 50.43 | +2.3% | -6.3% .. +11.7% | within noise |
|  | `configure-notify` | 66.65 | 63.69 | 101.58 | +47.0% | +37.5% .. +62.0% | slower |
|  | `fullscreen-state` | 44.84 | 41.45 | 49.31 | -1.1% | -8.6% .. +8.7% | within noise |
| focus_m2 | `main` | 41.84 | 36.45 | 54.94 |  |  | A/A floor ±22.1% |
|  | `main-again` | 38.95 | 36.95 | 52.34 | -6.9% | -22.1% .. +6.7% | A/A |
|  | `stable-primary` | 45.40 | 37.80 | 51.38 | +8.5% | -15.2% .. +23.7% | within noise |
|  | `stable-primary-1st` | 43.97 | 35.84 | 53.25 | +5.1% | -20.4% .. +26.6% | within noise |
|  | `configure-notify` | 50.12 | 38.85 | 58.19 | +19.8% | -8.4% .. +32.4% | within noise |
|  | `fullscreen-state` | 45.53 | 38.85 | 54.53 | +8.8% | -11.7% .. +22.9% | within noise |
| focus_m200 | `main` | 41.12 | 37.40 | 59.02 |  |  | A/A floor ±16.7% |
|  | `main-again` | 41.73 | 37.02 | 49.36 | +1.5% | -17.2% .. +15.3% | A/A |
|  | `stable-primary` | 42.10 | 39.50 | 51.15 | +2.4% | -14.9% .. +17.3% | within noise |
|  | `stable-primary-1st` | 39.26 | 37.16 | 57.77 | -4.5% | -20.0% .. +10.0% | within noise |
|  | `configure-notify` | 48.52 | 38.72 | 53.67 | +18.0% | -8.4% .. +29.9% | within noise |
|  | `fullscreen-state` | 43.56 | 36.36 | 52.94 | +5.9% | -15.7% .. +25.9% | within noise |
| focus_m3_u1000 | `main` | 38.11 | 35.66 | 51.62 |  |  | A/A floor ±30.6% |
|  | `main-again` | 45.37 | 37.03 | 51.67 | +19.0% | -1.0% .. +30.4% | A/A |
|  | `stable-primary` | 43.24 | 38.68 | 54.21 | +13.5% | -1.4% .. +31.1% | within noise |
|  | `stable-primary-1st` | 39.82 | 36.00 | 57.08 | +4.5% | -7.4% .. +33.6% | within noise |
|  | `configure-notify` | 43.45 | 37.12 | 56.63 | +14.0% | -1.6% .. +34.2% | within noise |
|  | `fullscreen-state` | 46.74 | 37.34 | 53.01 | +22.6% | -0.6% .. +32.7% | within noise |
| focus_m50 | `main` | 42.14 | 36.21 | 56.52 |  |  | A/A floor ±26.9% |
|  | `main-again` | 44.56 | 37.74 | 50.31 | +5.7% | -18.4% .. +26.9% | A/A |
|  | `stable-primary` | 44.50 | 37.87 | 54.35 | +5.6% | -14.3% .. +31.5% | within noise |
|  | `stable-primary-1st` | 43.66 | 37.62 | 53.95 | +3.6% | -18.9% .. +31.9% | within noise |
|  | `configure-notify` | 45.64 | 38.30 | 57.34 | +8.3% | -12.5% .. +35.2% | within noise |
|  | `fullscreen-state` | 40.99 | 37.28 | 56.79 | -2.7% | -17.0% .. +25.9% | within noise |
| fullscreen_redundant | `main` | 102.84 | 90.68 | 119.73 |  |  | A/A floor ±13.2% |
|  | `main-again` | 100.17 | 93.05 | 113.93 | -2.6% | -8.4% .. +13.3% | A/A |
|  | `stable-primary` | 101.58 | 89.97 | 119.20 | -1.2% | -8.7% .. +14.7% | within noise |
|  | `stable-primary-1st` | 104.69 | 97.08 | 113.29 | +1.8% | -3.4% .. +14.4% | within noise |
|  | `configure-notify` | 104.58 | 99.20 | 125.06 | +1.7% | -3.9% .. +14.4% | within noise |
|  | `fullscreen-state` | 116.87 | 108.97 | 134.62 | +13.6% | +7.7% .. +26.8% | within noise |
| fullscreen_toggle | `main` | 27.06 | 22.89 | 31.53 |  |  | A/A floor ±8.8% |
|  | `main-again` | 27.77 | 23.55 | 31.19 | +2.7% | -4.6% .. +8.6% | A/A |
|  | `stable-primary` | 27.46 | 26.13 | 30.21 | +1.5% | -0.9% .. +4.0% | within noise |
|  | `stable-primary-1st` | 27.56 | 24.94 | 29.79 | +1.9% | -1.1% .. +5.2% | within noise |
|  | `configure-notify` | 28.39 | 24.54 | 30.15 | +4.9% | -4.5% .. +8.3% | within noise |
|  | `fullscreen-state` | 38.25 | 34.88 | 46.25 | +41.4% | +35.1% .. +47.6% | slower |
| fullscreen_toggle_wait | `main` | 40.27 | 37.60 | 49.83 |  |  | A/A floor ±7.6% |
|  | `main-again` | 41.02 | 38.27 | 46.73 | +1.9% | -2.9% .. +7.5% | A/A |
|  | `stable-primary` | 41.30 | 38.03 | 45.00 | +2.5% | -3.8% .. +7.7% | within noise |
|  | `stable-primary-1st` | 40.81 | 38.84 | 46.97 | +1.3% | -3.1% .. +8.0% | within noise |
|  | `configure-notify` | 40.84 | 38.80 | 44.53 | +1.4% | -3.7% .. +5.1% | within noise |
|  | `fullscreen-state` | 51.56 | 48.14 | 64.50 | +28.0% | +22.5% .. +42.0% | slower |
| map_churn | `main` | 303.89 | 287.73 | 513.29 |  |  | A/A floor ±13.0% |
|  | `main-again` | 312.40 | 291.20 | 347.40 | +2.8% | -13.3% .. +6.3% | A/A |
|  | `stable-primary` | 325.56 | 299.85 | 358.03 | +7.1% | -11.6% .. +12.9% | within noise |
|  | `stable-primary-1st` | 315.02 | 282.22 | 354.17 | +3.7% | -13.5% .. +11.2% | within noise |
|  | `configure-notify` | 316.19 | 290.86 | 371.12 | +4.0% | -11.9% .. +13.4% | within noise |
|  | `fullscreen-state` | 308.03 | 291.77 | 392.71 | +1.4% | -14.3% .. +6.9% | within noise |
| output_enter_m200 | `main` | 69.44 | 59.15 | 92.49 |  |  | A/A floor ±24.2% |
|  | `main-again` | 68.09 | 59.79 | 101.44 | -2.0% | -23.4% .. +23.3% | A/A |
|  | `stable-primary` | 18.00 | 16.71 | 23.34 | -74.1% | -79.2% .. -66.8% | faster |
|  | `stable-primary-1st` | 19.77 | 16.50 | 22.99 | -71.5% | -78.6% .. -65.7% | faster |
|  | `configure-notify` | 65.84 | 60.25 | 88.99 | -5.2% | -23.7% .. +16.4% | within noise |
|  | `fullscreen-state` | 66.07 | 58.47 | 87.79 | -4.9% | -24.3% .. +17.5% | within noise |
| output_enter_m3 | `main` | 67.89 | 57.68 | 90.12 |  |  | A/A floor ±23.2% |
|  | `main-again` | 69.95 | 56.82 | 91.98 | +3.0% | -22.0% .. +22.2% | A/A |
|  | `stable-primary` | 20.53 | 16.52 | 23.43 | -69.8% | -78.1% .. -64.7% | faster |
|  | `stable-primary-1st` | 19.92 | 15.64 | 22.72 | -70.7% | -77.9% .. -66.8% | faster |
|  | `configure-notify` | 64.50 | 57.63 | 87.95 | -5.0% | -21.2% .. +20.8% | within noise |
|  | `fullscreen-state` | 79.46 | 59.94 | 87.22 | +17.0% | -6.3% .. +29.8% | within noise |
| output_enter_m3_u1000 | `main` | 98.55 | 76.49 | 112.77 |  |  | A/A floor ±23.2% |
|  | `main-again` | 93.09 | 76.83 | 107.78 | -5.5% | -20.4% .. +23.2% | A/A |
|  | `stable-primary` | 29.63 | 26.85 | 33.44 | -69.9% | -72.2% .. -60.7% | faster |
|  | `stable-primary-1st` | 27.75 | 25.43 | 34.43 | -71.8% | -73.6% .. -63.8% | faster |
|  | `configure-notify` | 89.06 | 78.96 | 116.20 | -9.6% | -18.1% .. +20.9% | within noise |
|  | `fullscreen-state` | 81.69 | 77.36 | 108.25 | -17.1% | -21.9% .. +19.4% | within noise |

## satellite CPU per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| barrier_only | `main` | 45.27 | 39.13 | 72.50 |  |  | A/A floor ±12.5% |
|  | `main-again` | 43.30 | 38.93 | 46.51 | -4.4% | -12.4% .. +8.5% | A/A |
|  | `stable-primary` | 40.33 | 38.36 | 47.41 | -10.9% | -14.5% .. +3.9% | within noise |
|  | `stable-primary-1st` | 43.14 | 39.34 | 48.62 | -4.7% | -11.6% .. +6.8% | within noise |
|  | `configure-notify` | 45.43 | 39.54 | 47.31 | +0.3% | -10.1% .. +13.1% | within noise |
|  | `fullscreen-state` | 43.17 | 39.28 | 49.00 | -4.7% | -12.1% .. +9.9% | within noise |
| configure_position | `main` | 7.04 | 6.86 | 7.73 |  |  | A/A floor ±3.1% |
|  | `main-again` | 7.06 | 6.91 | 7.46 | +0.3% | -3.1% .. +2.1% | A/A |
|  | `stable-primary` | 7.03 | 6.94 | 7.35 | -0.1% | -3.1% .. +1.3% | within noise |
|  | `stable-primary-1st` | 6.99 | 6.82 | 7.26 | -0.7% | -3.7% .. +1.0% | within noise |
|  | `configure-notify` | 21.36 | 17.19 | 22.33 | +203.4% | +158.6% .. +212.0% | slower |
|  | `fullscreen-state` | 7.02 | 6.85 | 7.35 | -0.3% | -3.6% .. +1.1% | within noise |
| configure_position_size | `main` | 23.82 | 21.39 | 28.11 |  |  | A/A floor ±6.6% |
|  | `main-again` | 23.58 | 21.56 | 25.70 | -1.0% | -6.1% .. +2.7% | A/A |
|  | `stable-primary` | 23.25 | 21.71 | 24.67 | -2.4% | -6.8% .. +0.8% | within noise |
|  | `stable-primary-1st` | 23.24 | 21.16 | 25.38 | -2.4% | -11.4% .. +2.9% | within noise |
|  | `fullscreen-state` | 23.36 | 21.41 | 25.25 | -1.9% | -8.1% .. +3.4% | within noise |
| configure_position_sync | `main` | 22.23 | 20.96 | 24.15 |  |  | A/A floor ±5.0% |
|  | `main-again` | 22.28 | 21.18 | 24.80 | +0.2% | -4.1% .. +5.1% | A/A |
|  | `stable-primary` | 21.85 | 20.81 | 23.20 | -1.7% | -6.1% .. +3.0% | within noise |
|  | `stable-primary-1st` | 22.40 | 21.20 | 23.77 | +0.8% | -4.7% .. +5.8% | within noise |
|  | `configure-notify` | 32.03 | 31.11 | 46.55 | +44.1% | +38.8% .. +54.1% | slower |
|  | `fullscreen-state` | 22.17 | 21.31 | 23.50 | -0.3% | -4.7% .. +5.1% | within noise |
| focus_m2 | `main` | 21.22 | 19.95 | 27.08 |  |  | A/A floor ±17.2% |
|  | `main-again` | 20.64 | 19.99 | 25.53 | -2.7% | -17.5% .. +4.5% | A/A |
|  | `stable-primary` | 22.94 | 20.74 | 25.76 | +8.1% | -13.4% .. +20.6% | within noise |
|  | `stable-primary-1st` | 22.57 | 19.89 | 25.68 | +6.3% | -15.3% .. +21.0% | within noise |
|  | `configure-notify` | 24.62 | 20.27 | 27.47 | +16.0% | -8.5% .. +23.9% | within noise |
|  | `fullscreen-state` | 23.64 | 20.65 | 25.50 | +11.4% | -12.5% .. +19.2% | within noise |
| focus_m200 | `main` | 21.64 | 20.11 | 27.52 |  |  | A/A floor ±13.6% |
|  | `main-again` | 21.75 | 20.28 | 24.92 | +0.5% | -12.3% .. +14.4% | A/A |
|  | `stable-primary` | 22.62 | 21.51 | 26.09 | +4.5% | -9.1% .. +12.1% | within noise |
|  | `stable-primary-1st` | 20.91 | 20.23 | 26.80 | -3.4% | -14.6% .. +5.0% | within noise |
|  | `configure-notify` | 24.13 | 20.88 | 26.36 | +11.5% | -6.9% .. +18.4% | within noise |
|  | `fullscreen-state` | 22.59 | 20.23 | 25.85 | +4.4% | -10.9% .. +17.4% | within noise |
| focus_m3_u1000 | `main` | 20.58 | 19.81 | 25.02 |  |  | A/A floor ±21.4% |
|  | `main-again` | 23.09 | 20.11 | 25.34 | +12.2% | -0.4% .. +21.3% | A/A |
|  | `stable-primary` | 22.57 | 21.34 | 27.19 | +9.7% | +2.3% .. +27.4% | within noise |
|  | `stable-primary-1st` | 20.92 | 19.89 | 26.56 | +1.7% | -5.1% .. +20.9% | within noise |
|  | `configure-notify` | 21.27 | 20.14 | 26.15 | +3.4% | -3.8% .. +24.4% | within noise |
|  | `fullscreen-state` | 23.06 | 20.11 | 25.68 | +12.1% | -1.6% .. +21.3% | within noise |
| focus_m50 | `main` | 21.46 | 20.00 | 26.65 |  |  | A/A floor ±19.6% |
|  | `main-again` | 22.68 | 20.34 | 25.26 | +5.7% | -11.3% .. +19.2% | A/A |
|  | `stable-primary` | 23.19 | 20.67 | 26.68 | +8.1% | -6.6% .. +25.8% | within noise |
|  | `stable-primary-1st` | 21.92 | 20.25 | 26.07 | +2.2% | -11.0% .. +23.4% | within noise |
|  | `configure-notify` | 22.45 | 20.40 | 26.53 | +4.6% | -9.0% .. +23.9% | within noise |
|  | `fullscreen-state` | 21.20 | 20.25 | 27.42 | -1.2% | -12.4% .. +14.6% | within noise |
| fullscreen_redundant | `main` | 56.80 | 50.46 | 63.56 |  |  | A/A floor ±13.2% |
|  | `main-again` | 53.48 | 51.20 | 60.90 | -5.9% | -8.8% .. +13.4% | A/A |
|  | `stable-primary` | 56.24 | 50.25 | 61.84 | -1.0% | -9.5% .. +13.4% | within noise |
|  | `stable-primary-1st` | 58.48 | 54.18 | 60.78 | +3.0% | -0.9% .. +14.3% | within noise |
|  | `configure-notify` | 56.47 | 52.14 | 66.64 | -0.6% | -5.2% .. +12.7% | within noise |
|  | `fullscreen-state` | 63.34 | 59.89 | 71.59 | +11.5% | +8.1% .. +25.3% | within noise |
| fullscreen_toggle | `main` | 15.83 | 13.55 | 16.72 |  |  | A/A floor ±6.7% |
|  | `main-again` | 16.22 | 13.98 | 16.98 | +2.4% | -3.0% .. +6.6% | A/A |
|  | `stable-primary` | 16.11 | 15.44 | 16.81 | +1.7% | -1.4% .. +5.4% | within noise |
|  | `stable-primary-1st` | 16.14 | 14.48 | 16.88 | +2.0% | -1.1% .. +5.7% | within noise |
|  | `configure-notify` | 16.06 | 14.66 | 16.66 | +1.5% | -3.7% .. +5.8% | within noise |
|  | `fullscreen-state` | 21.49 | 19.72 | 24.62 | +35.7% | +31.8% .. +40.6% | slower |
| fullscreen_toggle_wait | `main` | 27.05 | 26.24 | 30.25 |  |  | A/A floor ±3.7% |
|  | `main-again` | 27.31 | 26.43 | 28.86 | +1.0% | -2.2% .. +3.8% | A/A |
|  | `stable-primary` | 27.49 | 26.10 | 28.93 | +1.6% | -2.2% .. +4.2% | within noise |
|  | `stable-primary-1st` | 27.60 | 26.27 | 29.26 | +2.0% | -2.6% .. +5.3% | within noise |
|  | `configure-notify` | 27.38 | 26.54 | 28.23 | +1.2% | -2.1% .. +3.2% | within noise |
|  | `fullscreen-state` | 32.85 | 31.67 | 38.15 | +21.5% | +17.5% .. +27.4% | slower |
| map_churn | `main` | 165.84 | 158.67 | 218.49 |  |  | A/A floor ±7.8% |
|  | `main-again` | 166.65 | 160.39 | 175.08 | +0.5% | -7.8% .. +5.1% | A/A |
|  | `stable-primary` | 172.29 | 164.66 | 183.45 | +3.9% | -5.4% .. +9.3% | within noise |
|  | `stable-primary-1st` | 166.70 | 157.91 | 176.62 | +0.5% | -8.6% .. +5.3% | within noise |
|  | `configure-notify` | 168.26 | 162.53 | 186.49 | +1.5% | -7.5% .. +5.9% | within noise |
|  | `fullscreen-state` | 165.19 | 160.58 | 197.65 | -0.4% | -8.7% .. +2.3% | within noise |
| output_enter_m200 | `main` | 32.35 | 29.47 | 40.80 |  |  | A/A floor ±22.9% |
|  | `main-again` | 31.32 | 29.45 | 44.76 | -3.2% | -19.9% .. +19.1% | A/A |
|  | `stable-primary` | 10.08 | 9.69 | 12.74 | -68.8% | -74.3% .. -62.8% | faster |
|  | `stable-primary-1st` | 9.94 | 8.91 | 11.61 | -69.3% | -75.7% .. -62.5% | faster |
|  | `configure-notify` | 30.87 | 29.62 | 40.42 | -4.6% | -20.5% .. +12.6% | within noise |
|  | `fullscreen-state` | 30.98 | 29.28 | 40.09 | -4.2% | -20.5% .. +15.5% | within noise |
| output_enter_m3 | `main` | 32.11 | 29.50 | 39.47 |  |  | A/A floor ±20.9% |
|  | `main-again` | 32.65 | 29.56 | 41.28 | +1.7% | -19.0% .. +21.1% | A/A |
|  | `stable-primary` | 10.64 | 9.44 | 12.19 | -66.9% | -73.9% .. -61.6% | faster |
|  | `stable-primary-1st` | 10.32 | 8.87 | 11.80 | -67.9% | -75.1% .. -64.3% | faster |
|  | `configure-notify` | 31.24 | 29.55 | 40.32 | -2.7% | -18.9% .. +22.0% | within noise |
|  | `fullscreen-state` | 38.43 | 30.03 | 40.48 | +19.7% | -9.6% .. +26.8% | within noise |
| output_enter_m3_u1000 | `main` | 39.71 | 32.50 | 44.35 |  |  | A/A floor ±20.1% |
|  | `main-again` | 36.77 | 32.78 | 43.91 | -7.4% | -20.2% .. +15.4% | A/A |
|  | `stable-primary` | 13.96 | 12.96 | 16.02 | -64.9% | -68.5% .. -55.9% | faster |
|  | `stable-primary-1st` | 12.39 | 11.58 | 15.13 | -68.8% | -71.9% .. -61.3% | faster |
|  | `configure-notify` | 36.18 | 33.03 | 44.77 | -8.9% | -19.2% .. +22.7% | within noise |
|  | `fullscreen-state` | 33.70 | 32.86 | 43.90 | -15.1% | -21.3% .. +22.3% | within noise |

## Xwayland CPU per op (us)

| benchmark | tree | median | p10 | p90 | change vs baseline | 95% CI | verdict |
|---|---|---|---|---|---|---|---|
| barrier_only | `main` | 50.05 | 45.11 | 81.96 |  |  | A/A floor ±8.5% |
|  | `main-again` | 48.23 | 44.79 | 52.93 | -3.6% | -8.5% .. +6.2% | A/A |
|  | `stable-primary` | 47.19 | 44.53 | 52.92 | -5.7% | -10.3% .. +5.7% | within noise |
|  | `stable-primary-1st` | 48.49 | 44.92 | 56.95 | -3.1% | -9.0% .. +8.3% | within noise |
|  | `configure-notify` | 50.14 | 45.69 | 53.16 | +0.2% | -6.2% .. +10.4% | within noise |
|  | `fullscreen-state` | 49.78 | 45.88 | 56.66 | -0.5% | -7.8% .. +9.5% | within noise |
| configure_position | `main` | 6.80 | 6.65 | 7.51 |  |  | A/A floor ±3.1% |
|  | `main-again` | 6.79 | 6.66 | 7.13 | -0.1% | -3.0% .. +2.6% | A/A |
|  | `stable-primary` | 6.78 | 6.72 | 7.12 | -0.2% | -2.7% .. +1.4% | within noise |
|  | `stable-primary-1st` | 6.76 | 6.60 | 7.01 | -0.6% | -3.1% .. +1.8% | within noise |
|  | `configure-notify` | 24.79 | 19.93 | 26.33 | +264.8% | +209.0% .. +280.7% | slower |
|  | `fullscreen-state` | 6.78 | 6.60 | 7.11 | -0.2% | -2.9% .. +1.0% | within noise |
| configure_position_size | `main` | 55.48 | 49.23 | 65.90 |  |  | A/A floor ±8.1% |
|  | `main-again` | 54.13 | 50.13 | 60.79 | -2.4% | -8.1% .. +4.3% | A/A |
|  | `stable-primary` | 54.27 | 51.36 | 57.86 | -2.2% | -7.2% .. +0.2% | within noise |
|  | `stable-primary-1st` | 53.34 | 48.96 | 59.47 | -3.9% | -10.3% .. +1.5% | within noise |
|  | `fullscreen-state` | 54.07 | 50.11 | 59.82 | -2.5% | -7.6% .. +3.6% | within noise |
| configure_position_sync | `main` | 22.95 | 21.10 | 24.21 |  |  | A/A floor ±9.9% |
|  | `main-again` | 23.34 | 21.13 | 24.42 | +1.7% | -3.0% .. +9.9% | A/A |
|  | `stable-primary` | 21.74 | 20.98 | 23.69 | -5.3% | -7.9% .. +6.3% | within noise |
|  | `stable-primary-1st` | 23.58 | 21.35 | 24.39 | +2.7% | -2.5% .. +10.9% | within noise |
|  | `configure-notify` | 35.25 | 34.38 | 50.88 | +53.6% | +49.4% .. +74.2% | slower |
|  | `fullscreen-state` | 22.66 | 21.32 | 23.77 | -1.3% | -6.1% .. +8.3% | within noise |
| focus_m2 | `main` | 23.05 | 22.01 | 29.59 |  |  | A/A floor ±19.6% |
|  | `main-again` | 22.54 | 21.96 | 28.77 | -2.2% | -19.5% .. +8.4% | A/A |
|  | `stable-primary` | 24.05 | 22.12 | 28.11 | +4.4% | -17.1% .. +22.3% | within noise |
|  | `stable-primary-1st` | 24.23 | 21.82 | 28.58 | +5.1% | -17.7% .. +23.0% | within noise |
|  | `configure-notify` | 27.85 | 22.23 | 30.29 | +20.8% | -9.2% .. +27.7% | within noise |
|  | `fullscreen-state` | 25.97 | 22.61 | 29.34 | +12.7% | -13.9% .. +22.4% | within noise |
| focus_m200 | `main` | 23.61 | 22.16 | 30.14 |  |  | A/A floor ±15.0% |
|  | `main-again` | 23.65 | 22.36 | 28.00 | +0.1% | -14.7% .. +13.4% | A/A |
|  | `stable-primary` | 23.83 | 22.81 | 28.47 | +0.9% | -14.7% .. +6.3% | within noise |
|  | `stable-primary-1st` | 23.11 | 22.39 | 29.76 | -2.1% | -16.5% .. +5.0% | within noise |
|  | `configure-notify` | 27.17 | 22.92 | 29.35 | +15.1% | -8.0% .. +21.9% | within noise |
|  | `fullscreen-state` | 24.61 | 22.38 | 28.53 | +4.2% | -13.2% .. +17.4% | within noise |
| focus_m3_u1000 | `main` | 22.55 | 21.77 | 28.26 |  |  | A/A floor ±25.5% |
|  | `main-again` | 24.70 | 22.12 | 28.63 | +9.5% | -0.6% .. +25.3% | A/A |
|  | `stable-primary` | 23.15 | 22.11 | 28.98 | +2.7% | -2.5% .. +23.5% | within noise |
|  | `stable-primary-1st` | 23.07 | 21.86 | 29.43 | +2.3% | -4.1% .. +23.5% | within noise |
|  | `configure-notify` | 23.26 | 22.09 | 29.97 | +3.2% | -2.4% .. +22.4% | within noise |
|  | `fullscreen-state` | 24.81 | 22.08 | 29.08 | +10.0% | -1.3% .. +25.9% | within noise |
| focus_m50 | `main` | 23.14 | 21.97 | 30.05 |  |  | A/A floor ±23.0% |
|  | `main-again` | 24.56 | 22.21 | 28.57 | +6.1% | -11.5% .. +22.9% | A/A |
|  | `stable-primary` | 24.38 | 22.01 | 29.07 | +5.4% | -10.5% .. +26.0% | within noise |
|  | `stable-primary-1st` | 23.86 | 22.23 | 28.74 | +3.1% | -13.0% .. +26.1% | within noise |
|  | `configure-notify` | 24.53 | 22.30 | 30.34 | +6.0% | -9.0% .. +27.1% | within noise |
|  | `fullscreen-state` | 23.36 | 22.27 | 31.03 | +1.0% | -13.5% .. +15.5% | within noise |
| fullscreen_redundant | `main` | 63.11 | 58.13 | 71.59 |  |  | A/A floor ±9.4% |
|  | `main-again` | 61.67 | 58.72 | 68.82 | -2.3% | -6.4% .. +9.4% | A/A |
|  | `stable-primary` | 62.50 | 58.45 | 68.91 | -1.0% | -6.3% .. +9.8% | within noise |
|  | `stable-primary-1st` | 64.19 | 61.62 | 67.39 | +1.7% | -3.5% .. +9.9% | within noise |
|  | `configure-notify` | 64.33 | 61.00 | 75.64 | +1.9% | -2.8% .. +9.7% | within noise |
|  | `fullscreen-state` | 73.83 | 70.04 | 82.24 | +17.0% | +12.8% .. +25.9% | slower |
| fullscreen_toggle | `main` | 14.62 | 12.76 | 15.62 |  |  | A/A floor ±5.3% |
|  | `main-again` | 14.83 | 13.33 | 15.93 | +1.4% | -2.7% .. +5.4% | A/A |
|  | `stable-primary` | 14.85 | 14.34 | 15.39 | +1.6% | -0.9% .. +3.7% | within noise |
|  | `stable-primary-1st` | 14.76 | 13.65 | 15.64 | +0.9% | -1.4% .. +3.5% | within noise |
|  | `configure-notify` | 14.83 | 13.78 | 15.37 | +1.4% | -3.5% .. +4.7% | within noise |
|  | `fullscreen-state` | 21.27 | 19.95 | 24.21 | +45.5% | +41.9% .. +49.0% | slower |
| fullscreen_toggle_wait | `main` | 18.88 | 18.33 | 21.44 |  |  | A/A floor ±3.9% |
|  | `main-again` | 19.15 | 18.45 | 19.98 | +1.5% | -1.6% .. +3.4% | A/A |
|  | `stable-primary` | 19.22 | 18.26 | 20.05 | +1.8% | -2.8% .. +4.7% | within noise |
|  | `stable-primary-1st` | 19.28 | 18.34 | 20.42 | +2.1% | -2.8% .. +4.6% | within noise |
|  | `configure-notify` | 18.89 | 18.42 | 19.77 | +0.1% | -3.3% .. +2.2% | within noise |
|  | `fullscreen-state` | 25.39 | 24.79 | 28.89 | +34.5% | +30.0% .. +40.4% | slower |
| map_churn | `main` | 185.68 | 175.89 | 250.07 |  |  | A/A floor ±8.2% |
|  | `main-again` | 186.78 | 177.94 | 196.76 | +0.6% | -8.4% .. +4.1% | A/A |
|  | `stable-primary` | 192.96 | 183.69 | 207.64 | +3.9% | -6.3% .. +9.9% | within noise |
|  | `stable-primary-1st` | 186.73 | 175.73 | 201.08 | +0.6% | -10.5% .. +5.8% | within noise |
|  | `configure-notify` | 190.20 | 181.55 | 211.24 | +2.4% | -8.1% .. +7.4% | within noise |
|  | `fullscreen-state` | 185.10 | 180.82 | 220.96 | -0.3% | -9.1% .. +2.4% | within noise |
| output_enter_m200 | `main` | 41.81 | 38.06 | 53.02 |  |  | A/A floor ±20.8% |
|  | `main-again` | 40.59 | 38.07 | 54.75 | -2.9% | -21.1% .. +20.0% | A/A |
|  | `stable-primary` | 10.92 | 10.49 | 14.02 | -73.9% | -78.7% .. -68.6% | faster |
|  | `stable-primary-1st` | 11.68 | 10.48 | 14.15 | -72.1% | -78.1% .. -65.7% | faster |
|  | `configure-notify` | 40.50 | 38.47 | 52.07 | -3.2% | -20.8% .. +10.6% | within noise |
|  | `fullscreen-state` | 40.17 | 38.08 | 52.58 | -3.9% | -21.0% .. +12.2% | within noise |
| output_enter_m3 | `main` | 38.67 | 35.83 | 47.78 |  |  | A/A floor ±22.1% |
|  | `main-again` | 39.35 | 35.72 | 50.21 | +1.7% | -19.6% .. +21.6% | A/A |
|  | `stable-primary` | 10.73 | 9.20 | 12.89 | -72.2% | -79.1% .. -66.8% | faster |
|  | `stable-primary-1st` | 10.94 | 9.17 | 12.91 | -71.7% | -78.9% .. -68.4% | faster |
|  | `configure-notify` | 37.90 | 36.01 | 50.30 | -2.0% | -19.6% .. +23.7% | within noise |
|  | `fullscreen-state` | 46.72 | 36.94 | 49.08 | +20.8% | -12.2% .. +27.2% | within noise |
| output_enter_m3_u1000 | `main` | 61.63 | 52.45 | 68.12 |  |  | A/A floor ±17.2% |
|  | `main-again` | 58.11 | 52.49 | 67.20 | -5.7% | -17.2% .. +11.4% | A/A |
|  | `stable-primary` | 18.63 | 17.20 | 21.25 | -69.8% | -72.1% .. -63.8% | faster |
|  | `stable-primary-1st` | 18.81 | 17.22 | 22.85 | -69.5% | -72.4% .. -63.8% | faster |
|  | `configure-notify` | 58.27 | 53.13 | 69.28 | -5.5% | -16.4% .. +15.0% | within noise |
|  | `fullscreen-state` | 54.18 | 52.95 | 68.52 | -12.1% | -18.3% .. +13.7% | within noise |

## Full `cargo test --release` suite (s)

| tree | runs | median | p10-p90 | vs baseline | failed runs |
|---|---|---|---|---|---|
| `main` | 5 | 10.10 | 10.00-10.60 | +0.0% | 0 |
| `main-again` | 5 | 10.16 | 9.89-10.36 | +0.6% | 0 |
| `stable-primary` | 5 | 11.19 | 10.93-11.70 | +10.8% | 0 |
| `stable-primary-1st` | 5 | 10.34 | 10.16-10.62 | +2.4% | 0 |
| `configure-notify` | 5 | 10.60 | 10.44-10.85 | +4.9% | 0 |
| `fullscreen-state` | 5 | 10.85 | 10.57-11.04 | +7.4% | 0 |

