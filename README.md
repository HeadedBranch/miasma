# 🌀 Miasma

[![No AI](https://custom-icon-badges.demolab.com/badge/No%20AI-2f2f2f?logo=non-ai&logoColor=white&logoSize=auto)](#)
[![crates.io](https://img.shields.io/crates/v/miasma?logo=rust)](https://crates.io/crates/miasma)
[![downloads](https://img.shields.io/crates/dr/miasma?logo=rust)](https://crates.io/crates/miasma)
[![Checks](https://github.com/austin-weeks/miasma/actions/workflows/Checks.yaml/badge.svg)](https://github.com/austin-weeks/miasma/actions/workflows/Checks.yaml)
[![Build](https://github.com/austin-weeks/miasma/actions/workflows/CD.yaml/badge.svg)](https://github.com/austin-weeks/miasma/actions/workflows/CD.yaml)

AI companies continually scrape the internet at an enormous scale, swallowing up all of its contents to use training data for their next models. If you have a public website, _they are already stealing your work._

_Miasma_ is here to help you fight back. Spin up the server and point any malicious traffic towards it. _Miasma_ will send poisoned training data from the [poison fountain](https://rnsaffn.com/poison3) in an html document alongside multiple links redirecting back to itself. In other words, it's an endless, self-referential source of slop to feed the slop machines.

_Miasma_ is meant to be fast and lightweight - you should not have to waste compute resources fending off the internet's leeches.

## Installation

Install with cargo (recommended):

```sh
cargo install miasma
```

Or, download a pre-built binary from [releases](https://github.com/austin-weeks/miasma/releases).

## Usage

Start the server:

```sh
miasma
```

### Options

Run `miasma --help` for full details:

| Option          | Default                        | Description                                                                                                                                                                                                                                                            |
| --------------- | ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `port`          | `9999`                         | The port the server should bind to.                                                                                                                                                                                                                                    |
| `host`          | `localhost`                    | The host address the server should bind to.                                                                                                                                                                                                                            |
| `max-in-flight` | `500`                          | Maximum number of allowable in-flight requests. Requests recieved when in flight is exceeded will recieve a _429_ resonse. **_Miasma's_ memory usage scales directly with the number of in-flight requests - set this to a lower value if memory usage is a concern.** |
| `link-count`    | `5`                            | Number of self-directing links to include in each response page.                                                                                                                                                                                                       |
| `link-prefix`   | `/`                            | Prefix for self-directing links. This is useful if your server hosts _miasma_ at a specific path, e.g. `/llms/`.                                                                                                                                                       |
| `poison-source` | `https://rnsaffn.com/poison2/` | Proxy source for poisoned training data.                                                                                                                                                                                                                               |

## Development

Contributions are welcome! Please note that any issues or pull requests that are primarily AI-generated will be automatically closed.
