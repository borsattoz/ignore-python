import pathlib
from typing import Self


class Override:
    """
    Manages a set of overrides provided explicitly by the end user.

    See https://docs.rs/ignore/latest/ignore/overrides/struct.Override.html for more information.
    """


class OverrideBuilder:
    """
    Builds a matcher for a set of glob overrides.

    See https://docs.rs/ignore/latest/ignore/overrides/struct.OverrideBuilder.html for more information.
    """

    def __init__(self, path: pathlib.Path) -> None:
        """Create a new override builder."""

    def build(self) -> Override: ...

    def add(self, glob: str) -> Self: ...
