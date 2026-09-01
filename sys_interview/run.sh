#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
CXX="${CXX:-g++}"
FLAGS=(-O2 -std=c++17 -pthread)

run_one() {
  local id="$1"
  case "$id" in
    01) $CXX "${FLAGS[@]}" 01_hotpath.cpp -o /tmp/sys01 && /tmp/sys01 ;;
    02) $CXX "${FLAGS[@]}" 02_cache_line.cpp -o /tmp/sys02 && /tmp/sys02 ;;
    03) $CXX "${FLAGS[@]}" 03_spsc_ring.cpp -o /tmp/sys03 && /tmp/sys03 ;;
    04) $CXX "${FLAGS[@]}" 04_memory_order.cpp -o /tmp/sys04 && /tmp/sys04 ;;
    10) python3 10_latency_budget.py ;;
    11) $CXX "${FLAGS[@]}" 11_affinity.cpp -o /tmp/sys11 && /tmp/sys11 ;;
    12) $CXX "${FLAGS[@]}" 12_clock.cpp -o /tmp/sys12 && /tmp/sys12 ;;
    *) echo "unknown $id" >&2; exit 1 ;;
  esac
}

if [[ "${1:-}" != "" ]]; then
  run_one "$1"
  exit 0
fi

for id in 01 02 03 04 10 11 12; do
  echo "======== $id ========"
  run_one "$id"
done
