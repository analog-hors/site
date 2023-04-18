---
title: "Projects"
---

Yeah turns out I've done quite a few of these huh. I'm quite proud of some of these ones.

## [`cozy-chess`](https://github.com/analog-hors/cozy-chess)
```
$ cargo run --release --example perft -- 7
   Compiling cozy-chess v0.3.0
    Finished release [optimized] target(s) in 6.37s
     Running `target\release\examples\perft.exe 7`
3195901860 nodes in 10.05s (318045465 nps)
```
A fast Chess and Chess960 move generation library in Rust suitable for Chess engines.

## [`webperft`](https://analog-hors.github.io/webperft/)
<img src="img/webperft.png" style="width: 50%;" alt="webperft screenshot">

A Chess and Chess960 move generation testing utility that helps you easily check [`perft`](https://www.chessprogramming.org/Perft) results in the browser.

## [`tantabus`](https://github.com/analog-hors/tantabus)
A superhuman Chess and Chess960 engine written in Rust. Built on top of `cozy-chess`.

## [`lunatic-web`](https://analog-hors.github.io/lunatic-web/)
<img src="img/lunatic_web.png" style="width: 50%;" alt="lunatic-web screenshot">

A web client for a WebAssembly port of Lunatic, the much weaker predecessor to Tantabus.

## [`cold-clear-web`](https://analog-hors.github.io/cold-clear-web/)
<img src="img/cold_clear_web.png" style="width: 50%;" alt="cold-clear-web screenshot">

A web client for a WebAssembly port of MinusKelvin's [`cold-clear`](https://github.com/MinusKelvin/cold-clear) Tetris AI.

## When the
```python
w=lambda b:[m for m in[448,292,273,146,*b"TI8",0]if b[1]&m==m][0]
def d(b):[print(".XO"[b[0]>>i&1|b[1]*2>>i&2],end="  \n"[i%3])for i in range(9)]
def a(b):
 if w(b):return-.5
 u,v=b;m=511^u^v;p=m,0
 while c:=m&-m:m^=c;p=min(p,(a([v,u|c]),c))
 b[0]|=p[1];return-p[0]
t,*b=0,0,0
s=input("P: ")>"1"
while w(b)|t<1:s^=1;[a,d][s](b);u,v=b;b=[v,u|s<<int(s and input(">"))];t=v|b[1]==511
d([b[s],b[~s]])
print("LD"[t])
```
A Python program that does
