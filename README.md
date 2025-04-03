# plates [![Tests](https://github.com/Grazen0/plates/actions/workflows/tests.yml/badge.svg)][workflow-tests]

A neat CLI program to manage file (tem)plates. Plates allows you to create **templates** at `~/.config/plates/templates` and **render** them to any directory of your liking.

This project is _very much_ inspired from [copier]. I just wanted to see if I could build something like it in Rust.

<!-- TODO: add usage example -->

## Features

- [x] Placeholder support.
  - [x] Built-in placeholders (plates_dir, plates_dir_basename, etc.).
  - [x] Placeholders can use previous placeholders.
  - [x] Placeholder transformation (via shell commands).
  - [ ] Different placeholder types (number, select, etc.).
  - [x] Placeholders within directory names.
  - [ ] Custom prompt validation.
- [x] Shell completions
  - [ ] Show available templates
- [x] Man pages.

## Building

The project is managed with Cargo, so everything should work fine out of the box.

[copier]: https://github.com/copier-org/copier
[workflow-tests]: https://github.com/Grazen0/plates/actions/workflows/tests.yml
