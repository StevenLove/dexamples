# Purpose
This is an example project to explore Dioxus's window management

# Procedure
`cargo run`
The app should open two windows, one that says "Hello" and one that says "World"
Tap the operating system's [x] button to close a window, and then tap the other [x] to close the other window.

In dioxus 0.5, this works just fine and you can close either window first.
<video src='./0.5-example.mov' width=300>


In dioxus 0.6.0, the windows don't react to the [x] button until you have attempted to close each both of the windows. 

<video src='./0.6-example.mov' width=300>

# Attempted workaround
I wanted to try using `use_wry_event_handler` to manually handle the `WindowEvent::CloseRequested` event, but I am struggling to hear any events on windows other than the first one (the one labeled "Hello" in this case). Right now the demo app tracks `WindowEvent::Moved` and prints to the console. You can see that moving the "Hello" window prints events and no events are printed when you move the other window.
