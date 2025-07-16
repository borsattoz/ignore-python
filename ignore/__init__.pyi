import pathlib
from typing import Self

from ignore.overrides import Override


class Error(Exception):
    """Represents an error that can occur during operations."""


class IOError(Exception):
    """
    An error that occurs when doing I/O.

    Currently, the only case where this error is used is for operating
    system errors of type ENOENT.
    """

    errno: int
    """A numeric error code from the C variable errno."""

    filename: str
    """The file system path involved."""


class DirEntry:
    """
    A directory entry.

    See https://docs.rs/ignore/latest/ignore/struct.DirEntry.html for
    more information.
    """

    def path(self) -> pathlib.Path: ...

    def depth(self) -> int: ...


class WalkBuilder:
    """
    WalkBuilder builds a recursive directory iterator.

    See https://docs.rs/ignore/latest/ignore/struct.WalkBuilder.html
    for more information.
    """

    def __init__(self, path: pathlib.Path) -> None:
        """Create a new builder for a recursive directory iterator for the directory given."""

    def hidden(self, yes: bool) -> Self: ...

    def ignore(self, yes: bool) -> Self: ...

    def parents(self, yes: bool) -> Self: ...

    def git_ignore(self, yes: bool) -> Self: ...

    def git_global(self, yes: bool) -> Self: ...

    def git_exclude(self, yes: bool) -> Self: ...

    def require_git(self, yes: bool) -> Self: ...

    def overrides(self, overrides: Override) -> Self: ...

    def follow_links(self, yes: bool) -> Self: ...

    def same_file_system(self, yes: bool) -> Self: ...

    def max_depth(self, depth: int | None) -> Self: ...

    def max_filesize(self, filesize: int | None) -> Self: ...

    def add_custom_ignore_filename(self, file_name: str) -> Self: ...

    def add(self, path: pathlib.Path) -> Self: ...

    def add_ignore(self, path: pathlib.Path) -> None: ...

    def build(self) -> Walk: ...


class Walk:
    """
    Walk is a recursive directory iterator over file paths in one or more directories.

    See https://docs.rs/ignore/latest/ignore/struct.Walk.html for more
    information.
    """

    def __init__(self, path: pathlib.Path) -> None:
        """Creates a new recursive directory iterator for the file path given."""

    def __iter__(self) -> Self: ...

    def __next__(self) -> DirEntry:
        """
        Advances the iterator and returns the next value.

        :raises IOError: Currently, only when a ENOENT error happens
        (e.g. broken symlinks when following them)
        """
