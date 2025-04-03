# plates [![Tests](https://github.com/Grazen0/plates/actions/workflows/tests.yml/badge.svg)][workflow-tests]

A neat CLI program to manage file (tem)plates. Plates allows you to create **templates** at `~/.config/plates/templates` and **render** them to any directory of your liking.

This project is very much inspired from [copier]. I just wanted to see if I could build something like it in Rust.

<!-- TODO: add usage example -->

## Features

- [x] Placeholder support.
- [ ] Different placeholder types (number, select, etc).
- [x] Placeholders within directory names.
- [ ] Shell completions with available templates.
- [x] Man pages.

## Building

The project is managed with Cargo, so everything should work fine out of the box.

[copier]: https://github.com/copier-org/copier
[workflow-tests]: https://github.com/Grazen0/plates/actions/workflows/tests.yml
