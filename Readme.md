# Advent of Code 2022

* Website: [AoC 2022](https://adventofcode.com/2022)
* Demo: [previously-immortal-lacewing.edgecompute.app](https://previously-immortal-lacewing.edgecompute.app).

This year I'm starting somewhat late to the party.
Again I feel uncertain whether I'll stick to it for longer,
but I'm wondering what'll come of some experiments here.

## Execute stuff

This project makes use of the fastly CLI.
See [developer.fastly.com/learning/tools/cli](https://developer.fastly.com/learning/tools/cli) for installation instructions.

In addition to that this project depends on rust tooling being available on the system.
Namely `rustup`, `cargo`, `rustc` and similar should be present.

Using the fastli CLI we can serve the compute@edge setup locally:

```bash
fastly compute serve
```
