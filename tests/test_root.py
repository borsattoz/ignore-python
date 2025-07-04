import ignore
from ignore import WalkBuilder, Walk
from pathlib import Path
import pytest
from sys import platform

PATH = Path("./")

def test_build():
    walk = WalkBuilder(PATH).build()
    assert type(walk) == Walk
    assert len(list(walk)) >= 2

def test_flags():
    builder = (WalkBuilder(PATH)
               .hidden(True)
               .ignore(True)
               .parents(True)
               .git_ignore(True)
               .git_global(True)
               .git_exclude(True)
               .require_git(True)
               .follow_links(True)
               .same_file_system(True)
               )

    assert type(builder) == WalkBuilder

def test_max_depth():
    builder = (WalkBuilder(PATH)
               .max_depth(None)
               .max_depth(0)
               .max_depth(42)
               )

    assert type(builder) == WalkBuilder

    with pytest.raises(OverflowError):
        WalkBuilder(PATH).max_depth(-1)

def test_max_filesize():
    builder = (WalkBuilder(PATH)
               .max_filesize(42)
               )

    assert type(builder) == WalkBuilder

    with pytest.raises(OverflowError):
        WalkBuilder(PATH).max_filesize(-1)

def test_add_custom_ignore_filename():
    builder = WalkBuilder(PATH).add_custom_ignore_filename("foo")

    assert type(builder) == WalkBuilder

def test_add():
    builder = WalkBuilder(PATH).add(Path("../bar"))

    assert type(builder) == WalkBuilder

def test_add_ignore():
    builder = WalkBuilder(PATH)

    if platform == 'win32':
        pstr = "C:\\Windows"
    else:
        pstr = "/"

    with pytest.raises(ignore.Error):
        builder.add_ignore(Path(pstr))

