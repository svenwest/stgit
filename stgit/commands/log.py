# -*- coding: utf-8 -*-
from __future__ import (
    absolute_import,
    division,
    print_function,
    unicode_literals,
)

import os

from stgit import argparse
from stgit.argparse import opt
from stgit.commands import common
from stgit.lib import log
from stgit.out import out
from stgit.run import Run

__copyright__ = """
Copyright (C) 2006, Catalin Marinas <catalin.marinas@gmail.com>
Copyright (C) 2008, Karl Hasselström <kha@treskal.com>

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License version 2 as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, see http://www.gnu.org/licenses/.
"""

help = 'Display the patch changelog'
kind = 'stack'
usage = ['[options] [--] [<patches>]']
description = """
List the history of the patch stack: the stack log. If one or more
patch names are given, limit the list to the log entries that touch
the named patches.

"stg undo" and "stg redo" let you step back and forth in the patch
stack. "stg reset" lets you go directly to any state."""

args = [argparse.patch_range(argparse.applied_patches,
                             argparse.unapplied_patches,
                             argparse.hidden_patches)]
options = [
    opt(
        '-b',
        '--branch',
        args=[argparse.stg_branches],
        short='Use BRANCH instead of the default one',
    ),
    opt(
        '-d',
        '--diff',
        action='store_true',
        short='Show the refresh diffs',
    ),
    opt(
        '-n',
        '--number',
        type='int',
        short='Limit the output to NUMBER commits',
    ),
    opt(
        '-f',
        '--full',
        action='store_true',
        short='Show the full commit ids',
    ),
    opt(
        '-g',
        '--graphical',
        action='store_true',
        short='Run gitk instead of printing',
    ),
    opt(
        '--clear',
        action='store_true',
        short='Clear the log history',
    ),
]

directory = common.DirectoryHasRepositoryLib()


def show_log(stacklog, pathlim, num, full, show_diff):
    cmd = ['git', 'log']
    if num is not None and num > 0:
        cmd.append('-%d' % num)
    if show_diff:
        cmd.append('-p')
    elif not full:
        cmd.append('--pretty=format:%h   %aD   %s')
    cmd.extend([stacklog.sha1, '--'])
    cmd.extend(pathlim)
    Run(*cmd).run()


def func(parser, options, args):
    if options.branch:
        stack = directory.repository.get_stack(options.branch)
    else:
        stack = directory.repository.current_stack
    patches = common.parse_patches(args, list(stack.patchorder.all))
    logref = log.log_ref(stack.name)
    try:
        logcommit = stack.repository.refs.get(logref)
    except KeyError:
        out.info('Log is empty')
        return

    if options.clear:
        log.delete_log(stack.repository, stack.name)
        return

    stacklog = log.get_log_entry(stack.repository, logref, logcommit)
    pathlim = [os.path.join('patches', pn) for pn in patches]

    if options.graphical:
        for o in ['diff', 'number', 'full']:
            if getattr(options, o):
                parser.error('cannot combine --graphical and --%s' % o)
        cmd = ['gitk', stacklog.simplified.sha1, '--'] + pathlim
        # Discard the exit codes generated by SIGINT, SIGKILL, and SIGTERM.
        Run(*cmd).returns([0, -2, -9, -15]).run()
    else:
        show_log(stacklog.simplified, pathlim,
                 options.number, options.full, options.diff)
