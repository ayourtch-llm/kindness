use std::collections::HashSet;

// NFA-based regex engine

#[derive(Debug, Clone)]
enum Re {
    Literal(char),
    Dot,
    CharClass(Vec<ClassItem>, bool), // items, negated
    Concat(Vec<Re>),
    Alt(Box<Re>, Box<Re>),
    Star(Box<Re>),
    Plus(Box<Re>),
    Quest(Box<Re>),
}

#[derive(Debug, Clone)]
enum ClassItem {
    Single(char),
    Range(char, char),
}

fn parse(pattern: &str) -> Re {
    let chars: Vec<char> = pattern.chars().collect();
    let (re, _) = parse_alt(&chars, 0);
    re
}

fn parse_alt(chars: &[char], pos: usize) -> (Re, usize) {
    let (mut left, mut pos) = parse_concat(chars, pos);
    while pos < chars.len() && chars[pos] == '|' {
        let (right, new_pos) = parse_concat(chars, pos + 1);
        left = Re::Alt(Box::new(left), Box::new(right));
        pos = new_pos;
    }
    (left, pos)
}

fn parse_concat(chars: &[char], mut pos: usize) -> (Re, usize) {
    let mut parts = Vec::new();
    while pos < chars.len() && chars[pos] != ')' && chars[pos] != '|' {
        let (atom, new_pos) = parse_quantifier(chars, pos);
        parts.push(atom);
        pos = new_pos;
    }
    if parts.is_empty() {
        (Re::Concat(vec![]), pos)
    } else if parts.len() == 1 {
        (parts.remove(0), pos)
    } else {
        (Re::Concat(parts), pos)
    }
}

fn parse_quantifier(chars: &[char], pos: usize) -> (Re, usize) {
    let (atom, pos) = parse_atom(chars, pos);
    if pos < chars.len() {
        match chars[pos] {
            '*' => (Re::Star(Box::new(atom)), pos + 1),
            '+' => (Re::Plus(Box::new(atom)), pos + 1),
            '?' => (Re::Quest(Box::new(atom)), pos + 1),
            _ => (atom, pos),
        }
    } else {
        (atom, pos)
    }
}

fn parse_atom(chars: &[char], pos: usize) -> (Re, usize) {
    if pos >= chars.len() {
        return (Re::Concat(vec![]), pos);
    }
    match chars[pos] {
        '(' => {
            let (re, new_pos) = parse_alt(chars, pos + 1);
            // expect ')'
            let new_pos = if new_pos < chars.len() && chars[new_pos] == ')' {
                new_pos + 1
            } else {
                new_pos
            };
            (re, new_pos)
        }
        '[' => parse_char_class(chars, pos),
        '.' => (Re::Dot, pos + 1),
        ch => (Re::Literal(ch), pos + 1),
    }
}

fn parse_char_class(chars: &[char], pos: usize) -> (Re, usize) {
    // pos is at '['
    let mut i = pos + 1;
    let negated = if i < chars.len() && chars[i] == '^' {
        i += 1;
        true
    } else {
        false
    };

    let mut items = Vec::new();
    while i < chars.len() && chars[i] != ']' {
        let ch = chars[i];
        if i + 2 < chars.len() && chars[i + 1] == '-' && chars[i + 2] != ']' {
            items.push(ClassItem::Range(ch, chars[i + 2]));
            i += 3;
        } else {
            items.push(ClassItem::Single(ch));
            i += 1;
        }
    }
    if i < chars.len() && chars[i] == ']' {
        i += 1;
    }
    (Re::CharClass(items, negated), i)
}

// NFA states
#[derive(Debug, Clone)]
enum NfaState {
    Match(MatchCond, usize), // condition, next state
    Split(usize, usize),     // two branches
    Accept,
}

#[derive(Debug, Clone)]
enum MatchCond {
    Literal(char),
    Dot,
    CharClass(Vec<ClassItem>, bool),
}

impl MatchCond {
    fn matches(&self, ch: char) -> bool {
        match self {
            MatchCond::Literal(c) => ch == *c,
            MatchCond::Dot => true,
            MatchCond::CharClass(items, negated) => {
                let mut found = false;
                for item in items {
                    match item {
                        ClassItem::Single(c) => {
                            if ch == *c {
                                found = true;
                                break;
                            }
                        }
                        ClassItem::Range(lo, hi) => {
                            if ch >= *lo && ch <= *hi {
                                found = true;
                                break;
                            }
                        }
                    }
                }
                if *negated { !found } else { found }
            }
        }
    }
}

struct Nfa {
    states: Vec<NfaState>,
}

impl Nfa {
    fn new() -> Self {
        Nfa { states: Vec::new() }
    }

    fn add_state(&mut self, state: NfaState) -> usize {
        let id = self.states.len();
        self.states.push(state);
        id
    }
}

// Compile regex AST to NFA, returns (start_state, end_state)
// end_state is a placeholder that will be patched
fn compile(nfa: &mut Nfa, re: &Re) -> (usize, usize) {
    match re {
        Re::Literal(ch) => {
            let end = nfa.add_state(NfaState::Accept); // placeholder
            let start = nfa.add_state(NfaState::Match(MatchCond::Literal(*ch), end));
            (start, end)
        }
        Re::Dot => {
            let end = nfa.add_state(NfaState::Accept);
            let start = nfa.add_state(NfaState::Match(MatchCond::Dot, end));
            (start, end)
        }
        Re::CharClass(items, negated) => {
            let end = nfa.add_state(NfaState::Accept);
            let start = nfa.add_state(NfaState::Match(
                MatchCond::CharClass(items.clone(), *negated),
                end,
            ));
            (start, end)
        }
        Re::Concat(parts) => {
            if parts.is_empty() {
                let s = nfa.add_state(NfaState::Accept);
                return (s, s);
            }
            let (first_start, mut prev_end) = compile(nfa, &parts[0]);
            for part in &parts[1..] {
                let (next_start, next_end) = compile(nfa, part);
                // Patch prev_end to point to next_start
                patch(nfa, prev_end, next_start);
                prev_end = next_end;
            }
            (first_start, prev_end)
        }
        Re::Alt(left, right) => {
            let (l_start, l_end) = compile(nfa, left);
            let (r_start, r_end) = compile(nfa, right);
            let end = nfa.add_state(NfaState::Accept);
            patch(nfa, l_end, end);
            patch(nfa, r_end, end);
            let start = nfa.add_state(NfaState::Split(l_start, r_start));
            (start, end)
        }
        Re::Star(inner) => {
            let (i_start, i_end) = compile(nfa, inner);
            let end = nfa.add_state(NfaState::Accept);
            let start = nfa.add_state(NfaState::Split(i_start, end));
            patch(nfa, i_end, start);
            (start, end)
        }
        Re::Plus(inner) => {
            let (i_start, i_end) = compile(nfa, inner);
            let end = nfa.add_state(NfaState::Accept);
            let split = nfa.add_state(NfaState::Split(i_start, end));
            patch(nfa, i_end, split);
            (i_start, end)
        }
        Re::Quest(inner) => {
            let (i_start, i_end) = compile(nfa, inner);
            let end = nfa.add_state(NfaState::Accept);
            patch(nfa, i_end, end);
            let start = nfa.add_state(NfaState::Split(i_start, end));
            (start, end)
        }
    }
}

fn patch(nfa: &mut Nfa, state: usize, target: usize) {
    match &nfa.states[state] {
        NfaState::Accept => {
            nfa.states[state] = NfaState::Split(target, target);
        }
        _ => {}
    }
}

fn epsilon_closure(nfa: &Nfa, states: &HashSet<usize>) -> HashSet<usize> {
    let mut closure = states.clone();
    let mut stack: Vec<usize> = states.iter().copied().collect();
    while let Some(s) = stack.pop() {
        match &nfa.states[s] {
            NfaState::Split(a, b) => {
                if closure.insert(*a) {
                    stack.push(*a);
                }
                if closure.insert(*b) {
                    stack.push(*b);
                }
            }
            _ => {}
        }
    }
    closure
}

fn step(nfa: &Nfa, states: &HashSet<usize>, ch: char) -> HashSet<usize> {
    let mut next = HashSet::new();
    for &s in states {
        if let NfaState::Match(cond, target) = &nfa.states[s] {
            if cond.matches(ch) {
                next.insert(*target);
            }
        }
    }
    epsilon_closure(nfa, &next)
}

fn nfa_accepts(nfa: &Nfa, start: usize, accept: usize, text: &str) -> bool {
    let mut current = HashSet::new();
    current.insert(start);
    let mut current = epsilon_closure(nfa, &current);

    for ch in text.chars() {
        current = step(nfa, &current, ch);
        if current.is_empty() {
            return false;
        }
    }

    current.contains(&accept)
}

pub fn regex_match(pattern: &str, text: &str) -> bool {
    let re = parse(pattern);
    let mut nfa = Nfa::new();
    let (start, end) = compile(&mut nfa, &re);
    let accept = end;
    // Mark the end state as Accept
    nfa.states[accept] = NfaState::Accept;
    nfa_accepts(&nfa, start, accept, text)
}

fn nfa_find(nfa: &Nfa, start: usize, accept: usize, text: &str) -> Option<(usize, usize)> {
    let chars: Vec<char> = text.chars().collect();
    let byte_offsets: Vec<usize> = text.char_indices().map(|(i, _)| i).collect();
    let total_len = text.len();

    for start_idx in 0..=chars.len() {
        let mut current = HashSet::new();
        current.insert(start);
        let mut current = epsilon_closure(nfa, &current);

        let mut last_match: Option<usize> = None;

        if current.contains(&accept) {
            let byte_start = if start_idx < byte_offsets.len() {
                byte_offsets[start_idx]
            } else {
                total_len
            };
            last_match = Some(byte_start);
        }

        for end_idx in start_idx..chars.len() {
            current = step(nfa, &current, chars[end_idx]);
            if current.is_empty() {
                break;
            }
            if current.contains(&accept) {
                let byte_end = if end_idx + 1 < byte_offsets.len() {
                    byte_offsets[end_idx + 1]
                } else {
                    total_len
                };
                last_match = Some(byte_end);
            }
        }

        if let Some(byte_end) = last_match {
            let byte_start = if start_idx < byte_offsets.len() {
                byte_offsets[start_idx]
            } else {
                total_len
            };
            return Some((byte_start, byte_end));
        }
    }

    None
}

pub fn regex_find(pattern: &str, text: &str) -> Option<(usize, usize)> {
    let re = parse(pattern);
    let mut nfa = Nfa::new();
    let (start, end) = compile(&mut nfa, &re);
    nfa.states[end] = NfaState::Accept;
    nfa_find(&nfa, start, end, text)
}
