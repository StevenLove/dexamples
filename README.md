# Purpose
This repo demonstrates problematic behavior with the dioxus_toast crate.  
When run in a `tokio::task::LocalSet` task, the application hangs after 15-20 seconds.

# Procedure
`cargo run`
The app should open a window that counts up and displays toasts. It typically hangs after 13 seconds when it is displaying `26`.