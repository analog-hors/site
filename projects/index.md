---
basic:
  title: "Projects"
  desc: "Stuff I've worked on."
---

## [`Boychesser`](https://github.com/analog-hors/Boychesser)
<img src="img/boychesser.png" style="max-width: 80%; height: 15em; object-fit: contain;" alt="boychesser screenshot">

A very small yet strong C# chess engine and the winner of [Sebastian Lague's Tiny Chess Bots Challenge](https://www.youtube.com/watch?v=Ne40a5LkK6A). Around 2770 Elo on the [CCRL blitz list](https://www.computerchess.org.uk/ccrl/404/). Joint project with [MinusKelvin](https://minuskelvin.net/) and a few other friends.

<br>

## [`cozy-chess`](https://github.com/analog-hors/cozy-chess)
```
$ cargo run --release --example perft -- 7
   Compiling cozy-chess v0.3.0
    Finished release [optimized] target(s) in 6.37s
     Running `target\release\examples\perft.exe 7`
3195901860 nodes in 10.05s (318045465 nps)
```
A fast Chess and Chess960 move generation library written in Rust suitable for Chess engines.

<br>

## [`webperft`](https://analog-hors.github.io/webperft/)
<img src="img/webperft.png" style="max-width: 80%; height: 15em; object-fit: contain;" alt="webperft screenshot">

A Chess and Chess960 move generation testing utility that helps you easily check [`perft`](https://www.chessprogramming.org/Perft) results in the browser.

<br>

## [`tantabus`](https://github.com/analog-hors/tantabus)
A superhuman Chess and Chess960 engine written in Rust. Built on top of `cozy-chess`.

<br>

## [`lunatic-web`](https://analog-hors.github.io/lunatic-web/)
<img src="img/lunatic_web.png" style="max-width: 80%; height: 15em; object-fit: contain;" alt="lunatic-web screenshot">

A web client for a WebAssembly port of Lunatic, the much weaker predecessor to Tantabus.

<br>

## [`cold-clear-web`](https://analog-hors.github.io/cold-clear-web/)
<img src="img/cold_clear_web.png" style="max-width: 80%; height: 15em; object-fit: contain;" alt="cold-clear-web screenshot">

A web client for a WebAssembly port of MinusKelvin's [`cold-clear`](https://github.com/MinusKelvin/cold-clear) Tetris AI.

<br>

## [`pones`](https://github.com/analog-hors/pones)
A work in progress NES emulator written in Rust.

<br>

## [`ColdTaco`](https://github.com/analog-hors/pones)
<img src="img/cold_taco.png" style="max-width: 80%; height: 15em; object-fit: contain;" alt="screenshot of ColdTaco playing NES tetris">

A C# program that leverages the [Nintaco API](https://nintaco.com/) to get [`cold-clear`](https://github.com/MinusKelvin/cold-clear) to play NES Tetris.

<br>

## [`glowfish`](https://wasm4.org/play/glowfish-chess)
<img src="img/glowfish.png" style="max-width: 80%; height: 15em; object-fit: contain;" alt="glowfish screenshot">

A simple [WASM-4](https://wasm4.org/) chess game with an optional "Vs CPU" mode.

<br>

## [`nnue-rs`](https://github.com/analog-hors/nnue-rs)
A Rust library for the inference of [Stockfish NNUE](https://stockfishchess.org/blog/2020/introducing-nnue-evaluation) neural networks. 

<br>

## [`mnist-digit-nn`](https://github.com/analog-hors/mnist-digit-nn)
A simple neural network-based digit classifier based on the [MNIST database](https://en.wikipedia.org/wiki/MNIST_database). Written as an excerise to better understand backpropagation and neural networks.
