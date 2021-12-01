<h1 align="center"><a href="https://aoc.eons.io/">Advent of Code 2021</a></h1>
<p align="center">
    <a href="https://github.com/sondr3/advent-2021/actions"><img alt="GitHub Actions Status" src="https://github.com/sondr3/advent-2021/workflows/pipeline/badge.svg" /></a>
</p>

<p align="center">
    <b>My solutions to Advent of Code 2021</b>
</p>

# What

This entire thing is deployed using [Yew](https://yew.rs/) as a simple web
application. The entire thing is about 100 lines of Rust with a little HTML
and CSS sprinkled on top.

# How

Each day is written in its own file, with its own inputs and tests and once
everything passes I simply add it to the `run_day` function so the front-end
can solve it. Very easy.

# Why

Because overengineering is more fun than being productive.

# LICENSE

MIT.
