# Introduction of Nanlo

## Goal, in short

- Create a wasm-based front-end compiler library for Nanlo language.
  - i.e. the library should compile Nanlo source code to WebAssembly binary.
- Create a wasm-based runtime, independent from the front-end compiler.
	- It must contains all useful features, such as JSON, HTTP, RegExp, etc. in the runtime. (Battery included runtime.)
	- It should be allow REPL. (Dynamically load the script and give results.)
	- Also, GC is required.
- Pack them for most of environment.
	- For native environment, it'll be a single binary. Should be easy to install and use.
	- For web environment, the FE compiler will be wasm, runtime will be a JS which utilizes FE and the result wasm binary.

## Name of the project

난로 is a Korean word for a small stove.
It is written as "nanlo" in English, even though it is pronounced as "nal-lo".

Originally, I wanted to create a language that is small and simple but looks like ML (meta language such as OCaml, F# or Haskell).
I wished some good extension looks like `.ml`, and a language name whose abbreviation is the extension.
The first idea was "nano-ml" and the extension ".nnl". (Isn't it looks like 'ml'?)
However, the name "nano-ml" is too ambiguous, so I changed it to "nanlo", which may be more unique and abbreviates to ".nnl".

## Motivation of the project
This project is both personal and experimental.

I have often found myself frustrated with existing programming languages.
Personally, I prefer ML-style languages like OCaml and Haskell,
but they are often difficult to install and use.
OCaml’s OPAM doesn’t work well on Windows,
and even its installation process is cumbersome.
Haskell is slightly better in this regard,
but its compiler is quite slow and large.
Functional languages are excellent for writing concise and clean code,
but their interpreters and compilers are not always ideal for practical use.

What I’ve been looking for is a functional programming language
in a small, simple language
that can be executed with a minimal runtime.
In this context, Lua stood out as a promising option.
Lua (or LuaJIT) is distributed as a small, fast, and extensible single binary.
However, its syntax and semantics didn’t align with what I wanted.
I need a ML with a runtime like Lua.

Creating a new language from scratch is not easy.
I’ve attempted this several times, but it’s hard to bring a project to completion.
Writing a parser isn’t overly difficult,
and while optimization and code generation are challenging,
they aren’t impossible.
However, developing a good runtime takes significant time and effort.
A naive interpreter is easy to implement and supports cross-platform execution,
but a slow language is not useful in today’s context.
Building a JIT compiler complicates things further,
as I’d need to account for multiple architectures and operating systems.
While LLVM can simplify some aspects of this process,
the resulting system is neither small nor simple.

As I was reflecting on these challenges,
I realized that WebAssembly had become stable and well-standardized.
There were many WASM runtimes developed, such as wasmtime,
and they were smaller than my guess! (It was about 1MB.)
Also, if I use WASM, I can run the code on the web without any modification.
It looks like better than using JIT compiler toolkit such as cranelift,
because even I created a good JIT compiler, it is hard to run on the web.

I thought it would be a good idea to use WebAssembly
as an intermediate representation (IR),
allowing all complex optimizations and code generation
to be handled by the WebAssembly runtime.
This idea became the foundation of this project.

## Objective, in detail

### dokki, the compiler front-end

The temporary name, "dokki", is a Korean word for axe.
(Maybe, it'll used to make some firewood for my stove "nanlo", haha.)

The FE will be a library based on Rust.
Since I need interpreter, it should be able to run on where
the runtime can be used.

### seongnyang, the runtime

The temporary name, "seongnyang", is a Korean word for match.

There will be multiple runtimes based on the environment.

For native environment, it'll be a native library based on WASM runtime.
It'll implements required functions,
and takes the FE's output and run it.

For web environment, it'll be a JS library which utilizes the FE.
It'll take the FE's output and run it.

### nanlo CLI

The CLI will be the only binary of the project.
It'll be a single binary which contains the FE and the runtime.

### How each component works internally

- Launch the CLI or Web, which use FE and runtime.
  - They will create a runtime context at first.
	- They will take the source code in various ways (e.g. files, REPL, eval function, etc.)
- When the source code is given, the FE will compile it to WASM binary.
  - The FE will use the runtime context to compile the source code.
- The runtime will take the WASM binary and run it.
	- When create a wasm instance, the runtime will inject useful functions as imports.
	- The runtime will run the WASM binary, and set the result and outputs to the runtime context. (exports, etc.)
