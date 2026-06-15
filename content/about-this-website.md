---
title: "About this website"
description: "My Rust static site generator"
date: "2026-06-15"
---

Welcome to my revamped personal site!

Instead of doing anything productive, I decided to take down my old personal site replace it with a Rust-based static site generator.

## Motivation

My biggest fear at the moment is developing AI psychosis (a prime example of this is [gstack](https://github.com/garrytan/gstack)). I vibe code a lot, so I made this website (mostly by hand) in order to delay the inevitable atrophying of my brain.

I had also been delaying learning Rust for at least a year, and I thought that writing something simple like a static site generator was a good way to get started with the language.

## How the site generation works

It collects all the markdown files in `/content`, parses them using `pulldown-cmark` and `serde_yaml`, then writes them all to `/output`. Static files like `index.css` and all my images are copied directly into `/output`.

Paths are hardcoded into the source code.

Templating is done using `maud`, an awesome library that generates HTML through Rust macros.

## AI

Some of the code was written with the help of AI.

- AI was used to copy over some of the styling from my old personal site. It has since been heavily modified by hand.
- I used AI to aid me in learning Rust syntax. I learn best from example, so this was the most efficient way. In a perfect world, I would be fully relying on Stack Overflow. Apart from that, all the Rust code was written by hand.

I used just enough AI so that the experience wasn't sluggish. If I had used it any higher magnitude, I would have developed AI psychosis.
