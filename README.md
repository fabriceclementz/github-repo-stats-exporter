# GitHub Stats Prometheus exporter

A simple Prometheus exporter that exposes `open_issues_count` and `stargazers_count` from a GitHub repository.

## Usage

Start the exporter at `http://localhost:8000/metrics` to expose stats of the `rust-lang/rust` repository

```sh
gh-repo-stats-exporter --repository rust-lang/rust
```

Display all options

```sh
gh-repo-stats-exporter -h
```

## Building

```sh
cargo build
```
