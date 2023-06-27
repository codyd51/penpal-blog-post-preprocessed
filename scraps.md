Rather than take you through a tour of the _implementation_ of my new literate programming scheme by _using_ my literate programming scheme (which would be just a tad too meta for my sensibilities), I'll instead use this post to demonstrate `penpal`'s usage in the abstract.

I tried a version of this blog post in which I showed `penpal` features in tandem with implementing a Rust-based 2048 clone, but it's clearer to just focus on the `penpal` mechanisms than to look at them in tandem with a real implementation.

## Creating a file

Programs are generally structured as a series of type definitions, functions, and a _core runloop_ that drives work. When building a program I'll normally start off by sketching a no-op run-loop, then continue by defining the types and data model.

When using `penpal`, we'll also 'reserve the space' for the future growth of the code. For example, we might start off with a code block like the following:

<<define main
file: game/src/main.rs
lang: rust
###
<<imports>>

fn main() {
println!("Let's get started!");
<<set_up_board>>
<<main_runloop>>
}
>>
<<show main>>

Now this is interesting! We've defined a file, `game/src/main.rs`, that contains some basic boilerplate. We'll want to expand this file over time, though. Eventually, we'll need to include imports, type definitions, functions, and the game runloop. In this blog's source code for the above snippet, preprocessed by `penpal`, we reference other snippets that'll be filled in later to provide each of these pieces of content.

{{<named-code-block lang="text" filename="blog/penpal/index.md">}}
{{define main_rs
file: game/src/main.rs
lang: rust
###
{{dependency_imports}}
{{module_imports}}
{{module_declarations}}

fn main() {
println!("Game started!");
{{set_up_board}}
{{main_runloop}}
}
}}
{{show main_rs}}
{{</named-code-block>}}

To summarize, we `define` a snippet called `main_rs`, with a given source language and file path. In it, we embed a few other snippets which are yet to be defined:

* `dependency_imports`
* `module_imports`
* `module_declarations`
* `set_up_board`
* `main_runloop`

---


------

```text
{{<rawhtml>}}
{{define board_rs
file: game/src/board.rs
lang: rust
###
{{imports}}

fn main() {
    println!("Game started!");
    {{set_up_board}}
    {{main_runloop}}
}
}}
{{</rawhtml>}}
```

I don't know anything about running Rust in the browser, let alone about using Rust to draw to a DOM canvas. However, I've heard these acronyms:

* Wasm
* wGPU
* AAHHH

I [found a simple example](https://rustwasm.github.io/docs/wasm-bindgen/examples/2d-canvas.html) that shows how to structure a project such that some Rust code, compiled to target the Wasm VM, can draw content to an HTML `<canvas>` element.

First up, we've got some boilerplate to set up the project structure. We'll need to define a few files:

---

I could see that the result I wanted would involve lots of interspersed prose and code! I could see that I needed some way to keep the content organized, and to make sure it was always possible for the reader to follow along and wind up with a working, and useful, program.

Rather than take you through a tour of the _implementation_ of my new literate programming scheme by _using_ my literate programming scheme (which would be just a tad too meta for my sensibilities), I'll instead use this post to demonstrate `penpal`'s usage by implementing something arbitrary and straightforward: A Rust-based 2048 clone that runs in the browser.


This meant that I was embedding lots of code blocks!

## A Rust-based 2048 clone that runs in the browser?!

I wrote out that sentence, so now I'm committed. Let's get started!

I don't know anything about running Rust in the browser, let alone about using Rust to draw to a DOM canvas. However, I've heard these acronyms:

* Wasm
* wGPU
* AAHHH

I [found a simple example](https://rustwasm.github.io/docs/wasm-bindgen/examples/2d-canvas.html) that shows how to structure a project such that some Rust code, compiled to target the Wasm VM, can draw content to an HTML `<canvas>` element.

First up, we've got some boilerplate to set up the project structure. We'll need to define a few files:

# Needs to have a 'reset' button so you can test merging tiles even if you were randomly spawned without them
# Needs to handle game over state?

Vision Pro SDK came out

TODO: 
Open source 2048 and penpal
Fix bugs in penpal
CSS on button?
Fix button icon?
