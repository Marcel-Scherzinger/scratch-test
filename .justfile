# converts a sb3 to a json file whenever the sb3 changes
watch-sb3 FILE:
	watchexec -w {{FILE}} cargo run --bin scratch-extract -- {{ FILE }} -o

