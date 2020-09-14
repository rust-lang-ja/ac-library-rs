#!/usr/bin/python3

import sys
import getopt

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
    -c  --output_comment    output comment
    -t  --output_test       output test code
    -h  --help              print help
'''
output_header = '//https://github.com/rust-lang-ja/ac-library-rs\n'
opt_list = ['output-comment', 'output-test', 'help']
output_list_all = ('lazysegtree', 'segtree', 'convolution', 'twosat', 'scc',
                   'fenwicktree', 'math', 'modint', 'maxflow', 'dsu', 'mincostflow', 'string', 'internal_bit', 'internal_math', 'internal_type_traits', 'internal_scc', 'internal_queue')
dependency_list = {'lazysegtree': ('internal_bit',), 'segtree': ('internal_bit',), 'convolution': ('internal_bit', 'modint',), 'math': ('internal_math',), 'modint': (
    'internal_math', 'internal_type_traits'), 'fenwicktree': ('internal_type_traits',), 'twosat': ('internal_scc',), 'scc': ('internal_scc',), 'maxflow': ('internal_queue', 'internal_type_traits',), 'mincostflow': ('internal_type_traits',)}
src_path = 'src/'


def output_file(filename, output_comment, output_test):
    global src_path

    res = []
    with open(src_path+filename+'.rs', 'r') as f:
        res.append('mod {}{{'.format(filename))

        for line in f:
            if not output_test and line.strip() == '#[cfg(test)]':
                # TODO
                # Find more better way.
                break
            if not output_comment and line.strip().startswith("//"):
                # TODO
                # Find more better way.
                continue
            res.append(line.rstrip())

        res.append('}')
    return res


try:
    opts, args = getopt.getopt(sys.argv[1:], 'tch', opt_list)
except getopt.GetoptError as e:
    print(e)
    print(usage)
    sys.exit(2)

output_comment = False
output_test = False

for o, v in opts:
    if o == '--output-comment' or o == '-c':
        output_comment = True
    if o == '--output-test' or o == '-t':
        output_test = True
    if o == '--help' or o == '-h':
        print(usage)
        sys.exit(0)

output_list = set()

while len(args) != 0:
    pop = args.pop()
    if not pop in output_list_all:
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
    buf = output_file(i, output_comment, output_test)
    output_data.extend(buf)

for i in output_list:
    # Modules that begin with 'internal' are for internal use, so they are not declared.
    if not i.startswith('internal'):
        output_data.append('use {}::*;'.format(i))

print(output_header)
for i in output_data:
    print(i)
