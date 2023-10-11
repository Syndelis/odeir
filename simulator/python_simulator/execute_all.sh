#!/usr/bin/env sh
cd $(dirname $(realpath $0))
"python3" "simulate.py" "--dt" "0.1" "--tf" "10" "--output" "output.csv" "--y0" "100" "50" "--ode-params" "1.1" "0.9" "1" "0.8"
"python3" "plot.py" "--input-file" "output.csv" "--output-file" "output.png" "--components" "x" "y"
