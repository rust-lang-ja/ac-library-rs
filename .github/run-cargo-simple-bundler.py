#!/usr/bin/python3

from tempfile import TemporaryDirectory
from pathlib import Path
import subprocess
from subprocess import PIPE
from argparse import ArgumentParser
import platform

MODULES = [
    'convolution',
    'dsu',
    'fenwicktree',
    'lazysegtree',
    'math',
    'maxflow',
    'mincostflow',
    'modint',
    'scc',
    'segtree',
    'string',
    'twosat',
]


def main() -> None:
    ArgumentParser().parse_args()

    manifest_path = Path(__file__).absolute().parent.parent \
        .joinpath('Cargo.toml')

    with TemporaryDirectory(prefix='ac-library-rs-run-cargo-simple-bundler-',
                            ) as tempdir:
        tempdir = Path(tempdir)

        for module in MODULES:
            rs = tempdir.joinpath(f'with-{module}.rs')

            with open(rs, 'a') as file:
                file.write(f'use ac_library_rs::{module} as _; fn main() {{}}')
                file.flush()

                output = subprocess.run(
                    ['cargo', 'simple-bundler', '--manifest-path',
                     manifest_path, '-e', rs], check=True, stdout=PIPE,
                ).stdout.decode()
                file.write(output)

            output = tempdir.joinpath('a')
            if platform.system() == 'Windows':
                output = output.with_suffix('.exe')
            subprocess.run(['rustc', '--edition', '2018', '-o', output, rs],
                           check=True)


if __name__ == '__main__':
    main()
