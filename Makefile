compile:
	cargo b

echo: compile
	maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 10

unique-ids: compile
	maelstrom test -w unique-ids --bin target/debug/unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition

broadcast: compile
	maelstrom test -w broadcast --bin ./target/debug/broadcast --node-count 5 --time-limit 20 --rate 10

broadcast-part: compile
	maelstrom test -w broadcast --bin target/debug/broadcast --node-count 5 --time-limit 20 --rate 10 --nemesis partition

web:
	maelstrom serve

fmt :
	cargo +nightly fmt

clippy :
	cargo clippy --all-features --all-targets -- -D warnings

