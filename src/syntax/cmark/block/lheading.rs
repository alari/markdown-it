// lheading (---, ===)
//
use crate::MarkdownIt;
use crate::block;
use crate::renderer;
use crate::syntax::base::core::inline::InlineNodes;
use crate::token::{Token, TokenData};

#[derive(Debug)]
pub struct SetextHeader {
    pub level: u8,
    pub marker: char,
}

impl TokenData for SetextHeader {
    fn render(&self, token: &Token, f: &mut renderer::Formatter) {
        static TAG : [&str; 2] = [ "h1", "h2" ];
        debug_assert!(self.level >= 1 && self.level <= 2);
        f.open(TAG[self.level as usize - 1]).contents(&token.children).close(TAG[self.level as usize - 1]).lf();
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.block.ruler.add("lheading", rule)
        .before("paragraph")
        .after_all();
}

fn rule(state: &mut block::State, silent: bool) -> bool {
    if silent { return false; }

    // if it's indented more than 3 spaces, it should be a code block
    if state.line_indent(state.line) >= 4 { return false; }

    let start_line = state.line;
    let mut next_line = start_line;
    let mut level = 0;

    'outer: loop {
        next_line += 1;

        if next_line >= state.line_max || state.is_empty(next_line) { break; }

        // this would be a code block normally, but after paragraph
        // it's considered a lazy continuation regardless of what's there
        if state.line_indent(next_line) >= 4 { continue; }

        //
        // Check for underline in setext header
        //
        if state.line_indent(next_line) >= 0 {
            let mut chars = state.get_line(next_line).chars().peekable();
            if let Some(marker @ ('-' | '=')) = chars.next() {
                while Some(&marker) == chars.peek() { chars.next(); }
                while let Some(' ' | '\t') = chars.peek() { chars.next(); }
                if chars.next().is_none() {
                    level = if marker == '=' { 1 } else { 2 };
                    break 'outer;
                }
            }
        }

        // quirk for blockquotes, this line should already be checked by that rule
        if state.s_count[next_line] < 0 { continue; }

        // Some tags can terminate paragraph without empty line.
        let old_state_line = state.line;
        state.line = next_line;
        for rule in state.md.block.ruler.iter() {
            if rule(state, true) {
                state.line = old_state_line;
                break 'outer;
            }
        }
        state.line = old_state_line;
    }


    if level == 0 {
        // Didn't find valid underline
        return false;
    }

    let content = state.get_lines(start_line, next_line, state.blk_indent, false).trim().to_owned();

    state.line = next_line + 1;

    let mut token = state.push(SetextHeader {
        level,
        marker: if level == 2 { '-' } else { '=' }
    });
    token.map = Some([ start_line, next_line ]);
    token.children.push(Token::new(InlineNodes {
        content
    }));

    true
}
