# Applause

![You deserve applause!](/images/applause.svg "You deserve applause!")

**WARNING: Applause is not ready for use.  It may not function yet.**

## Table of Contents

- [Applause](#applause)
  - [Table of Contents](#table-of-contents)
  - [Overview](#overview)
    - [Clap's Legacy](#claps-legacy)
    - [Why Applause?](#why-applause)
  - [Development Information](#development-information)
    - [Goals](#goals)
    - [Non-Goals](#non-goals)

## Overview

The Applause crate is the logical successor to
[Clap](https://github.com/clap-rs/clap). It is intended to be a more flexible,
more powerful, and more ergonomic command line argument parser.

*Note: Applause is created by a completely different group and not, at this point, endorsed by Clap or its maintainers.*

### Clap's Legacy

Clap allowed Rust programmers to annotate an existing struct with attributes to define the command line interface. Clap then generated the parser needed to read command line parameters into that struct without any additional code. The approach was relatively simple and flexible.

### Why Applause?

Applause is the logical successor of Clap. Clap was a great tool and deserves recognition. We considered using the name Clap2 in homage but thought that might be misconstrued as a new version of Clap. We decided to go with Applause to still pay tribute to Clap while also implying this crate is something special (worthy of applause).

## Development Information

### Goals

- [ ] Match Most Clap Capabilities
- [ ] Minimized Boilerplate through Opinionated Defaults (Batteries Included..just works without config)
  - [ ] Defaults to ALL Features Enabled
  - [ ] Defaults each Functionality to Automatically Configured and Running
- [ ] ```Config``` Parser & Generator Derive Macro
  - [ ] Parses and Generates to/from multiple sources
    - [ ] Defaults
    - [ ] Database
      - [ ] SQLite
      - [ ] MySQL
      - [ ] PostgreSQL
      - [ ] Key-Value Store
    - [ ] Config Files
      - [ ] TOML
      - [ ] YAML
      - [ ] JSON
      - [ ] XML
      - [ ] INI
      - [ ] .ENV
    - [ ] Environment Variables
    - [ ] Command Line Interface
  - [ ] Modular Parser/Generator (can insert sub-parsers into existing parsers)
  - [ ] Deriving ```Config``` generates documentation from doc comments during compile
    - [ ] CLI Help
    - [ ] Man Page
    - [ ] Markdown
  - [ ] Field attributes allow parsing/generating customization
    - [ ] Format Strings
    - [ ] RegEx
    - [ ] Custom Function-Based Parsing
    - [ ] Template File Specification
    - [ ] Validation
    - [ ] Type Conversion
    - [ ] Sub-Parser/Generator Insertion
- [ ] Feature Gate everything that isn't required to be in the core
- [ ] Easy Feature Addition by Developers (Plugins)
- [ ] Integration with Other Crates (behind Feature Gates)
  - [ ] [Log](https://crates.io/crates/log)
  - [ ] [Tracing](https://crates.io/crates/tracing)
  - [ ] [Serde](https://crates.io/crates/serde)
  - [ ] [Rand](https://crates.io/crates/rand)
  - [ ] [FastRand](https://crates.io/crates/fast_rand)
  - [ ] [BitMan](https://crates.io/crates/bitman)
  - [ ] [BitFlags](https://crates.io/crates/bitflags)
  - [ ] [DashMap](https://crates.io/crates/dashmap)
  - [ ] [Time](https://crates.io/crates/time)
  - [ ] [Chrono](https://crates.io/crates/chrono)
  - [ ] [HumanTime](https://crates.io/crates/humantime)
  - [ ] [IndexMap](https://crates.io/crates/indexmap)
  - [ ] [DashMap](https://crates.io/crates/dashmap)
  - [ ] [TinyVec](https://crates.io/crates/tinyvec)
  - [ ] [Nom](https://crates.io/crates/nom)
  - [ ] [Rayon](https://crates.io/crates/rayon)
  - [ ] [Heck](https://crates.io/crates/heck)
  - [ ] [Percent-Encoding](https://crates.io/crates/percent-encoding)
  - [ ] [TextWrap](https://crates.io/crates/textwrap)
  - [ ] [TermColor](https://crates.io/crates/termcolor)
  - [ ] [Actix-Web](https://crates.io/crates/actix-web)
  - [ ] [Hyper](https://crates.io/crates/hyper)
  - [ ] [Reqwest](https://crates.io/crates/reqwest)
  - [ ] [Diesel](https://crates.io/crates/diesel)
  - [ ] [Basteh](https://crates.io/crates/basteh)
  - [ ] [Garde](https://crates.io/crates/garde)
  - [ ] [UUID](https://crates.io/crates/uuid)
  - [ ] [Hex](https://crates.io/crates/hex)
  - ...and too many others to remember right now...
- [ ] 100% Documentation Coverage
- [ ] 100% Test Coverage
- [ ] 100% Example Coverage
- [ ] Video Tutorials

### Non-Goals

- Support for Every Possible Use Case
- Backwards Compatibility with Clap
