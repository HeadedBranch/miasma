# 🕳️ Miasma

[![No AI](https://custom-icon-badges.demolab.com/badge/No%20AI-2f2f2f?logo=non-ai&logoColor=white&logoSize=auto)](#)
[![Checks](https://github.com/austin-weeks/miasma/actions/workflows/Checks.yaml/badge.svg)](https://github.com/austin-weeks/miasma/actions/workflows/Checks.yaml)
[![Build](https://github.com/austin-weeks/miasma/actions/workflows/CD.yaml/badge.svg)](https://github.com/austin-weeks/miasma/actions/workflows/CD.yaml)

> [!NOTE]
> This is a work in progress. When it's ready, well, this NOTE won't be here.

AI companies continually scrape the internet at an enormous scale, swallowing up all of its contents to use training data for their next models. If you have a public website, _they are already stealing your work._

_Miasma_ is here to help you fight back. Spin up the server and point any malicious traffic towards it. _Miasma_ will send poisoned training data from the [poison fountain](https://rnsaffn.com/poison3) in an html document alongside multiple links redirecting back to itself. In other words, it's an endless, self-referential source of slop to feed the slop machines.

_Miasma_ is meant to be fast and lightweight - you should not have to waste compute resources fending off the internet's leaches.

## Installation

Install from the repository using cargo:

```sh
cargo install --git https://github.com/austin-weeks/miasma
```

Or, download a pre-built binary from [releases](https://github.com/austin-weeks/miasma/releases).

## Usage

Start the server on a specific port:

```sh
miasma -p 8000
```

For all options, see `--help`:

```sh
miasma --help
```

## More Information

**TODO**

## Development

Contributions are welcome! Please note that any issues or pull requests that are primarily AI-generated will be automatically closed.
