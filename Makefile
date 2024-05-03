run:
	cargo watch -x run

benchmarks:
	wrk -t12 -c400 -d30s --latency http://localhost:8181/healthz