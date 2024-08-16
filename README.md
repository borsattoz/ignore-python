# ignore in Python
This is a Python library that binds to the Rust crate
[ignore](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore).

ignore's Python bindings can be used for building a fast recursive
directory iterator that respects various filters such as globs, file
types and `.gitignore` files.

## Example
This example shows the most basic usage of this package. This code
will recursively traverse the current directory while automatically
filtering out files and directories according to ignore globs found in
files like `.ignore` and `.gitignore`:

```python
from ignore import Walk

for entry in Walk("./"):
	print(entry.path())
```

## Example: advanced
By default, the recursive directory iterator will ignore hidden files and directories. This can be disabled by building the iterator with `WalkBuilder`:

```python
from ignore import WalkBuilder

for entry in WalkBuilder("./").hidden(False).build():
	print(entry.path())
```

Refer to the [API documentation](https://borsattoz.github.io/ignore-python) for more information.

## How to install (from pip)
```sh
pip install ignore-python
# or
python -m pip install ignore-python
```

## How to develop
This assumes that you have rust and cargo installed. I use the
workflow recommended by [pyo3](https://github.com/PyO3/pyo3) and
[maturin](https://github.com/PyO3/maturin).
