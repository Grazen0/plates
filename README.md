# plates [![Tests](https://github.com/Grazen0/plates/actions/workflows/tests.yml/badge.svg)][workflow-tests]

A neat CLI program to manage file (tem)plates. Plates allows you to create **templates** at `~/.config/plates/templates` and **render** them to any directory of your liking.

This project is _very much_ inspired from [copier]. I just wanted to see if I could build something like it in Rust.

## Usage example

> [!NOTE]
> For more complete examples, you can check out [my own plates config][config-example].

Each **template** is a directory located at `$XDG_CONFIG_HOME/plates/templates`. You may include a `_plates.yml` file to define, among other things, **placeholders** for your template.

For a simple Python template, take the following directory structure for `~/.config/plates`

```
templates
└── python-starter/
    ├── src/
    │   └── main.py
    └── _plates.yml
```

Your `_plates.yml` file could look something like this:

```yaml
placeholders:
  - name: author_name
    message: "What's your name? "
  - name: project_name
    message: "Project name: "
    default: "{{ plates_dir_basename }}" # Will expand to the name of the directory being rendered to
  - name: creation_date
    message: "Project creation date: "
    default:
      # type can be one of "str", "shell", "env".
      type: shell
      value: "date +%Y-%m-%d"
```

Then, any file inside your template (such as`main.py`) may use these placeholders. For example:

```python
# This is project "{{ project_name }}", by {{ author_name }}.
# Created at {{ creation_date }}.
# Use for Good, not Evil.


def main():
    print("Hello, {{ author_name }}!")
```

You may now run the following command:

```bash
plates render my-new-project
```

You'll be prompted for the template you want to render and any placeholders it may have. Then, the template will be rendered at a (perhaps new) directory called "my-new-project".

> [!TIP]
> You can list all currently available templates with `plates list`.

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

If you use [Nix][nix], you already know what to do. Otherwise, this project is managed with Cargo, so you should be just a `cargo build` away from building this locally.

[copier]: https://github.com/copier-org/copier
[workflow-tests]: https://github.com/Grazen0/plates/actions/workflows/tests.yml
[config-example]: https://github.com/Grazen0/nixos-config/tree/main/profiles/home/ttymax/programs/plates/config/templates/cpp-cmake
[nix]: https://nixos.org/
