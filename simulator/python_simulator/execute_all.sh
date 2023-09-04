python3 simulate.py \
	--ode-params 0.1 0.1 0.1 0.1 0.1 \
	--y0 100 100 5 0 0 0 \
	--tf 50 \
	--dt 0.1 \
	| python3 plot.py \
		--output-file ".local/inhibitive_competition.png"

