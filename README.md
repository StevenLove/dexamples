# Purpose
This is an example project to explore Dioxus's message passing between Rust and javascript using `eval`, `dioxus.send` and `dioxus.recv`

# Procedure
`cargo run`
tap the `Send timestamps in a loop` button
watch as local_timestamps and remote_timestamps fill with timestamps as a message is passed from Rust->js->Rust
eventually, within 1 minute, it breaks. A call to `eval.recv().await` returns an `Err(Finished)` which you can see in the CLI logs.

# Questions
- Is there a better practice with Dioxus that will allow me to make RPC-style calls to javascript, passing in some args and getting back a return value
- Is there a problem with using `eval` repeatedly (100s of times) in a loop?
- Is this a timing issue that could be resolved by adding sleeps or timeouts?

# Version
Dioxus 0.5.6
not using `dx` Dioxus CLI