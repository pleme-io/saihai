#!/usr/bin/env python3
"""Convert the saihai spec's action tables into the tatara-lisp catalog.

The spec enumerates 284 actions across category sections. This reads them and
emits `(defaction …)` rows.

FAITHFUL from the spec (the load-bearing axes):
  - id, category, kind (M/O), authority rung (L0..L3)
  - observability: the E column (Obs/Blind) decides Class, because
    `:observed` empty <=> Blind. That is the property the whole border rests on.

DERIVED, and said out loud rather than implied:
  - gloss: title-cased from the id. The spec's tables carry no gloss column.
  - observed domain/field: domain from the category, field from the action's
    object. The CLASS is faithful; the specific field name is a routing detail
    refined per-action as backends land.
  - param kind: inferred from the spec's type annotation.
  - required: everything except a known-optional set, then required-first
    ordering, because the border rejects a required param after an optional one
    (Go and positional Python cannot express it).
"""
import re
import sys
import pathlib

SPEC = pathlib.Path(sys.argv[1])
OUT = pathlib.Path(sys.argv[2])

# Category section headers: "### 3.1 window (31)" / "### 3.12 session/login (…)"
SECTION = re.compile(r'^###\s+3\.\d+\s+([a-z][a-z &/—-]*?)\s*(?:\(|—|$)')
# The E cell carries markdown emphasis on the blind rows (`**Blind**`), so
# `\w+` matched none of them and silently DROPPED all 30 — which is how a
# conversion loses exactly the rows that matter most.
ROW = re.compile(r'^\|(\d+)\|`([a-z][a-z0-9-]*)`\|([^|]*)\|\*{0,2}([MO])\*{0,2}\|\*{0,2}(L[0-3])\*{0,2}\|([^|]+)\|')

# A row whose rung/observability are expressions over other actions' rows.
COMPUTED_ROW = re.compile(
    r'^\|(\d+)\|`([a-z][a-z0-9-]*)`\|([^|]*)\|\*{0,2}([MO])\*{0,2}\|'
    r'\s*(?:max|min)\([a-z]+\)\s*\|')

# The border's closed Category set.
CATEGORIES = {
    'window', 'workspace', 'layout', 'output', 'input', 'focus', 'session',
    'login', 'seat', 'launch', 'clipboard', 'capture', 'notify', 'theme', 'system',
}
# Section names that map onto a border category.
ALIAS = {
    'gesture': 'input', 'rule': 'window', 'hook': 'system', 'decoration': 'window',
    'launcher': 'launch', 'layer': 'window', 'term': 'launch', 'pane': 'window',
    'browser': 'launch', 'file': 'launch', 'audio': 'system', 'notification': 'notify',
    # ★ THE PLURAL AND THE FULL WORD ARE DIFFERENT KEYS, and getting that wrong
    # drops a whole section in SILENCE. The section regex takes the header's
    # first word, so "3.22 terminal (25)" yields `terminal` (not `term`) and
    # "3.25 files (10)" yields `files` (not `file`) — both missed the alias
    # table, both set category=None, and every row under them was skipped by
    # the `if not category` guard with no diagnostic. 35 of the 36 rows this
    # conversion lost were these two sections; the counts in the spec's own
    # totals table (terminal 25 · files 10) are what made it findable.
    'terminal': 'launch', 'files': 'launch',
    'keyboard': 'input', 'binding': 'input', 'monitor': 'output', 'display': 'output',
}

# Reserved in at least one target language — the border rejects these outright.
RENAME = {
    # ★ A RENAME TARGET MUST NOT ITSELF BE RESERVED. Two of these were:
    # `go -> goto` and `fn -> function` (goto is a C/Go/Java keyword,
    # function a JS/PHP one). The first was caught only because
    # `file-bookmark-go` landed and the border refused the derived field; the
    # second is still latent until a param named `fn` appears. The border owns
    # the reserved union and refuses either way — this table just must not
    # feed it another reserved word and call that a fix.
    'type': 'kind', 'class': 'category', 'func': 'callback', 'range': 'span',
    'map': 'mapping', 'match': 'criteria', 'from': 'source', 'in': 'inside',
    'is': 'state', 'as': 'alias', 'if': 'when', 'for': 'target', 'go': 'destination',
    'var': 'name', 'select': 'choice', 'new': 'fresh', 'delete': 'remove',
    'self': 'own', 'super': 'parent', 'return': 'result', 'import': 'source',
    'def': 'define', 'pass': 'skip', 'none': 'nothing', 'interface': 'iface',
    'package': 'pkg', 'chan': 'channel', 'defer': 'deferred', 'else': 'otherwise',
    'while': 'until', 'impl': 'implementation', 'trait': 'behaviour',
    'struct': 'record', 'enum': 'choice', 'fn': 'handler', 'let': 'binding',
    'mut': 'mutable', 'const': 'constant', 'lambda': 'closure', 'async': 'deferred',
    'await': 'wait', 'yield': 'emit', 'true': 'yes', 'false': 'no',
    # `with` is a Python keyword and was missing from the border's list, so it
    # emitted `def launch_open_path(with: str)` — invalid Python. Found by
    # running the generated module.
    'with': 'using', 'not': 'negate', 'and': 'both', 'or': 'either',
    'try': 'attempt', 'except': 'onerror', 'finally': 'atend', 'raise': 'signal',
    'assert': 'require', 'global': 'shared', 'this': 'current', 'throw': 'signal',
    'catch': 'onerror', 'do': 'perform', 'export': 'expose', 'extends': 'derives',
    'function': 'callable', 'null': 'absent', 'undefined': 'unset', 'void': 'empty',
    'public': 'open', 'private': 'closed', 'protected': 'guarded', 'static': 'fixed',
    'switch': 'choose', 'case': 'branch', 'default': 'fallback', 'loop': 'repeat',
    'crate': 'unit', 'mod': 'module', 'use': 'uses', 'where': 'given', 'ref': 'reference',
    'move': 'relocate', 'unsafe': 'raw', 'extern': 'external', 'dyn': 'dynamic',
    'elif': 'orwhen', 'del': 'drop', 'nonlocal': 'outer', 'fallthrough': 'cascade',
    'goto': 'jumpto', 'debugger': 'debug', 'instanceof': 'isa', 'typeof': 'typeof_',
    'implements': 'fulfils', 'break': 'stop', 'continue': 'resume', 'do_': 'perform',
}
OPTIONAL = {
    'wrap', 'follow', 'note', 'filter', 'path', 'variant', 'scope', 'mode',
    'output', 'workspace', 'refresh', 'body', 'urgency', 'selection', 'name',
    'on_nonempty', 'timeout', 'reason', 'label', 'icon', 'index', 'flags',
}

def param_kind(name: str, ty: str) -> str:
    t = (ty or '').lower()
    n = name.lower()
    if 'selector' in t or n in {'target', 'other', 'window', 'workspace', 'output', 'criteria'}:
        return 'selector'
    if 'direction' in t or n == 'direction':
        return 'direction'
    if t in {'bool', 'yesno'} or n in {'on', 'wrap', 'follow', 'enabled', 'locked', 'visible'}:
        return 'bool'
    if n in {'x', 'y', 'width', 'height', 'scale', 'rate', 'delay', 'vt', 'index',
             'inner', 'outer', 'refresh', 'amount', 'timeout', 'count'} or t in {'i32', 'u32', 'int'}:
        return 'int'
    return 'str'

def norm_ident(raw: str) -> str | None:
    n = re.sub(r'[^a-z0-9_]', '_', raw.strip().lower())
    n = re.sub(r'_+', '_', n).strip('_')
    if not n or not n[0].isalpha():
        return None
    return RENAME.get(n, n)

def parse_params(cell: str):
    out, seen = [], set()
    for part in (p.strip() for p in cell.split(',')):
        if not part:
            continue
        part = part.split('(')[0].strip()
        if ':' in part:
            raw, ty = part.split(':', 1)
        else:
            raw, ty = part, ''
        name = norm_ident(raw)
        if not name or name in seen:
            continue
        seen.add(name)
        out.append((name, param_kind(name, ty), name not in OPTIONAL))
    # The border rejects a required param after an optional one, so order
    # required-first. Stable within each group, so the spec's order survives.
    return [p for p in out if p[2]] + [p for p in out if not p[2]]

# Where the last-word heuristic is simply wrong about WHAT an action sets.
#
# `theme-select` sets the theme's NAME, not a field called "select";
# `session-lock` sets `locked`; `input-bind` sets the binding TABLE. The
# heuristic reads the verb instead of the object, and the failure is visible
# rather than silent — a declaration wanting `theme.name` reported
# `Gap::NoAction` the moment the generated catalog replaced the hand-authored
# one, which is how this was caught.
OBSERVED_OVERRIDE = {
    'theme-select': ('theme', 'name'),
    'theme-reload': ('theme', 'revision'),
    'session-lock': ('session', 'locked'),
    'session-unlock': ('session', 'locked'),
    'input-bind': ('inputs', 'bindings'),
    'input-unbind': ('inputs', 'bindings'),
    # The spec's real ids, not the provisional ones the hand-authored catalog
    # used. The spec's enumeration is authoritative; a stale override key
    # produces a visible Gap::NoAction rather than a silent miss.
    'input-set-keyboard-repeat': ('inputs', 'repeat'),
    'input-set-keyboard-layout': ('inputs', 'layout'),
    'input-set-mode': ('inputs', 'mode'),
    'output-set-enabled': ('outputs', 'enabled'),
    'output-set-scale': ('outputs', 'scale'),
    'output-set-mode': ('outputs', 'mode'),
    'window-focus': ('focus', 'window'),
    'workspace-focus': ('focus', 'workspace'),
}

def observed_for(action_id: str, category: str, blind: bool):
    if blind:
        return []
    if action_id in OBSERVED_OVERRIDE:
        return [OBSERVED_OVERRIDE[action_id]]
    domain = {
        'window': 'windows', 'workspace': 'workspaces', 'layout': 'layout',
        'output': 'outputs', 'input': 'inputs', 'focus': 'focus',
        'session': 'session', 'login': 'session', 'seat': 'session',
        'launch': 'windows', 'clipboard': 'session', 'capture': 'session',
        'notify': 'session', 'theme': 'theme', 'system': 'session',
    }[category]
    # Field from the action's object: `window-set-fullscreen` -> `fullscreen`.
    parts = action_id.split('-')
    field = parts[-1] if len(parts) > 1 else 'state'
    field = re.sub(r'[^a-z0-9_]', '_', field)
    if not field or not field[0].isalpha():
        field = 'state'
    return [(domain, RENAME.get(field, field))]

def gloss_for(action_id: str) -> str:
    words = action_id.split('-')
    return (words[0].capitalize() + ' ' + ' '.join(words[1:])).strip()

def main():
    text = SPEC.read_text().splitlines()
    category = None
    rows, seen_ids = [], set()
    for line in text:
        m = SECTION.match(line)
        if m:
            raw = m.group(1).strip().lower()
            # Headers carry qualifiers: "input — devices", "session & power",
            # "login — PAM", "seat & VT", "pane / multiplexer". The category is
            # the first word; everything after it is a section label.
            first = re.split(r'[^a-z]+', raw)[0]
            category = first if first in CATEGORIES else ALIAS.get(first)
            continue
        # A row whose R/E cells are COMPUTED rather than literal. `action-chain`
        # is the only one: the spec gives it `max(steps)` / `min(steps)`, which
        # no static catalog row can carry, so ROW's `(L[0-3])` group did not
        # match and it dropped silently alongside the two sections above.
        #
        # The conservative static rendering is the correct one, not a fudge: a
        # chain's authority is its steps' maximum and its observability their
        # minimum, and at catalog time the steps are unknown. L3 + blind is the
        # only rendering that cannot be wrong in the dangerous direction — it
        # never lets a chain smuggle a high-rung step past a low-rung loop, and
        # never lets "the chain ran" be mistaken for observable state.
        c = COMPUTED_ROW.match(line)
        if c and category and c.group(2) not in seen_ids:
            aid = c.group(2)
            seen_ids.add(aid)
            rows.append({
                'id': aid, 'gloss': gloss_for(aid), 'category': category,
                'kind': 'mutate', 'auth': 'l3', 'params': parse_params(c.group(3)),
                'observed': [],
            })
            continue

        r = ROW.match(line)
        if not r or not category:
            continue
        _, aid, params, kind, rung, eff = r.groups()
        if aid in seen_ids:
            continue
        seen_ids.add(aid)
        blind = 'blind' in eff.lower().strip('* ')
        k = 'observe' if kind == 'O' else 'mutate'
        # A reader that observes nothing is a validate error, and rightly:
        # a read that reads nothing is a typo. Force readers observable.
        if k == 'observe':
            blind = False
        rows.append({
            'id': aid, 'gloss': gloss_for(aid), 'category': category, 'kind': k,
            'auth': rung.lower(), 'params': parse_params(params),
            'observed': observed_for(aid, category, blind),
        })

    body = []
    for r in rows:
        ps = '\n'.join(
            f'           (defparam :name "{n}" :kind :{k} :required {"#t" if req else "#f"})'
            for n, k, req in r['params'])
        obs = '\n'.join(
            f'             (defobserved :domain :{d} :field "{f}")'
            for d, f in r['observed'])
        body.append(
            f'(defaction :id "{r["id"]}" :gloss "{r["gloss"]}"\n'
            f'  :category :{r["category"]} :kind :{r["kind"]} :auth :{r["auth"]}\n'
            f'  :params ({ps.lstrip() if ps else ""})\n'
            f'  :observed ({obs.lstrip() if obs else ""}))')

    header = f''';; ─────────────────────────────────────────────────────────────────────
;; THE DESKTOP ACTION CATALOG — {len(rows)} actions.
;;
;; GENERATED from docs/saihai-desktop-action-spec.md §3 by
;; scratchpad/convert_catalog.py, then committed. Regenerating is how new rows
;; land; hand-editing one row is fine, hand-editing many means the spec and the
;; catalog have diverged.
;;
;; ── WHAT IS FAITHFUL, AND WHAT IS DERIVED ───────────────────────────
;; Faithful from the spec, because these are the load-bearing axes:
;;   id, category, kind (mutate/observe), authority rung, and OBSERVABILITY —
;;   the spec's Obs/Blind column decides whether :observed is populated, and
;;   an empty :observed IS the statement that an action is blind.
;;
;; Derived, and said out loud rather than implied:
;;   :gloss              title-cased from the id; the spec's tables carry no
;;                       gloss column.
;;   :observed field     domain from the category, field from the action's
;;                       object. The CLASS is faithful; the specific field name
;;                       is a routing detail, refined per-action as real
;;                       backends land. A wrong field name shows up as a
;;                       Gap::NoAction — visible — not as silent success.
;;   :required           everything outside a known-optional set, ordered
;;                       required-first because the border rejects a required
;;                       parameter after an optional one (Go and positional
;;                       Python cannot express it).
;;
;; ── THE ONE RULE AN AUTHOR MUST UNDERSTAND ──────────────────────────
;;   :kind :observe                  -> Read        (never planned)
;;   :kind :mutate + observed rows   -> Converging  (reconciler may converge)
;;   :kind :mutate + observed EMPTY  -> Blind       (may fire, NEVER counted)
;;
;; There is deliberately no :observability field. An author names what can be
;; read back; nobody gets to assert convergeability.
;;
;; ── AUTHORITY ────────────────────────────────────────────────────────
;;   :l0 unprivileged · :l1 seat-owner · :l2 controller · :l3 break-glass
;; ─────────────────────────────────────────────────────────────────────

'''
    OUT.write_text(header + '\n\n'.join(body) + '\n')
    from collections import Counter
    # ★ THE DENOMINATOR, CHECKED. This conversion lost 36 rows silently and
    # nothing said so — the artifact simply had 248 in it and looked complete.
    # The spec carries its own total; compare against it and refuse rather than
    # write a short catalog.
    declared = int(re.search(r'\*\*TOTAL\*\* *\| *\*\*(\d+)\*\*', SPEC.read_text()).group(1))
    if len(rows) != declared:
        missing = declared - len(rows)
        raise SystemExit(
            f'REFUSING to write: converted {len(rows)} rows, the spec declares '
            f'{declared} ({missing:+d}). A short catalog reads as complete; find '
            f'the dropped section or row first.')
    print(f'{len(rows)} actions (spec declares {declared})')
    print('by category:', dict(Counter(r['category'] for r in rows)))
    print('by kind    :', dict(Counter(r['kind'] for r in rows)))
    print('blind      :', sum(1 for r in rows if not r['observed'] and r['kind'] == 'mutate'))

main()
