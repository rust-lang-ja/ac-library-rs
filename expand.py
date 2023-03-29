#!/usr/bin/python3

import sys
import getopt
import tempfile
import subprocess

usage = '''Usage:expand.py [options] <output modules>
Output Modules:
    convolution
    dsu
    fenwicktree
    lazysegtree
    math
    maxflow
    mincostflow
    modint
    scc
    segtree
    string
    twosat

You can select multiple modules for <output modules>
    e.g.)expand.py math segtree

Options:
    -a  --all       import all modules
    -h  --help      print help
    -o file
    --output file   output file
'''
output_header = '//https://github.com/rust-lang-ja/ac-library-rs\n'
opt_list = ['help', 'all', 'output=']
output_list_all = ('convolution', 'dsu', 'fenwicktree', 'lazysegtree', 'math',
                   'maxflow',  'mincostflow', 'modint', 'scc',  'segtree',
                   'string', 'twosat',
                   'internal_bit', 'internal_math', 'internal_queue',
                   'internal_scc', 'internal_type_traits',)
dependency_list = {'convolution': ('internal_bit', 'modint',),
                   'lazysegtree': ('internal_bit', 'segtree'),
                   'math': ('internal_math',),
                   'maxflow': ('internal_type_traits', 'internal_queue',),
                   'mincostflow': ('internal_type_traits',),
                   'modint': ('internal_math',),
                   'scc': ('internal_scc',),
                   'segtree': ('internal_bit', 'internal_type_traits',),
                   'twosat': ('internal_scc',), }
src_path = 'src/'
output_path = None


def output_file(filename):
    global src_path

    res = []
    with open(src_path+filename+'.rs', 'r', encoding='utf-8', newline='') as f:
        res.append('pub mod {} {{'.format(filename))

        for line in f:
            res.append(line.rstrip())

        res.append('}')
    return res


try:
    opts, args = getopt.getopt(sys.argv[1:], 'aho:', opt_list)
except getopt.GetoptError as e:
    print(e)
    print(usage)
    sys.exit(2)

if len(opts) == 0 and len(args) == 0:
    print(usage)
    sys.exit(0)

for o, v in opts:
    if o == '--help' or o == '-h':
        print(usage)
        sys.exit(0)
    elif o == '--all' or o == '-a':
        args = list(output_list_all)
    elif o == '--output' or o == '-o':
        output_path = v

output_list = set()

while len(args) != 0:
    pop = args.pop()
    if pop not in output_list_all:
        print('invalid args:{}'.format(pop))
        print(usage)
        sys.exit(2)
    output_list.add(pop)
    if pop in dependency_list:
        for d in dependency_list[pop]:
            args.append(d)

output_list = list(output_list)
output_list.sort()

output_data = []
for i in output_list:
    buf = output_file(i)
    output_data.extend(buf)

for i in output_list:
    # Modules that begin with 'internal' are for internal use, so they are not
    # declared.
    if not i.startswith('internal'):
        output_data.append('use {}::*;'.format(i))

# rustfmt
with tempfile.TemporaryDirectory() as temp_dir:
    temp_file = temp_dir + '/output.rs'
    with open(temp_file, 'w', encoding='utf-8', newline='') as f:
        print(output_header, file=f)
        for i in output_data:
            print(i, file=f)
    output_data = subprocess.run(["rustfmt", temp_file], check=True)
    with open(temp_file, 'r', encoding='utf-8', newline='') as f:
        wf = open(output_path, 'w', encoding='utf-8', newline='') if output_path is not None else sys.stdout
        for line in f:
            print(line, end='', file=wf)
        if output_path is not None:
            wf.close()
