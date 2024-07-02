use anyhow::bail;

/// A node of a cmake configure file.
#[derive(Debug)]
pub enum Node {
    Text(String),
    Variable(String),
    CmakeDefine { key: String, value: String },
}

pub fn parse_cmake_configure(input: &str) -> anyhow::Result<Vec<Node>> {
    enum State {
        Text { buffer: String },
        Variable { buffer: String },

        CmakeDefineStart,
        CmakeDefineKey { buffer: String },
        CmakeDefineValue { key: String, buffer: String },
    }

    let mut ret = Vec::new();
    let mut state = State::Text {
        buffer: String::new(),
    };
    let mut char_iter = input.chars();
    while let Some(ch) = char_iter.next() {
        let cmakedefine = "cmakedefine";

        match (&mut state, ch) {
            (State::Text { buffer }, '@') => {
                ret.push(Node::Text(std::mem::take(buffer)));
                state = State::Variable {
                    buffer: String::new(),
                };
            }
            (State::Text { buffer }, '#') if char_iter.as_str().starts_with(cmakedefine) => {
                for _ in 0..cmakedefine.len() {
                    let _ = char_iter.next().is_some();
                }

                ret.push(Node::Text(std::mem::take(buffer)));
                state = State::CmakeDefineStart;
            }
            (State::Text { buffer }, ch) => {
                buffer.push(ch);
            }
            (State::Variable { buffer }, '@') => {
                ret.push(Node::Variable(std::mem::take(buffer)));
                state = State::Text {
                    buffer: String::new(),
                };
            }
            (State::Variable { buffer }, ch) => {
                buffer.push(ch);
            }
            (State::CmakeDefineStart, ' ') => {
                // Consume whitespace.
            }
            (State::CmakeDefineStart, ch) => {
                state = State::CmakeDefineKey {
                    buffer: String::from(ch),
                };
            }
            (State::CmakeDefineKey { buffer }, ' ') => {
                state = State::CmakeDefineValue {
                    key: std::mem::take(buffer),
                    buffer: String::new(),
                };
            }
            (State::CmakeDefineKey { buffer }, ch) => {
                buffer.push(ch);
            }
            (State::CmakeDefineValue { buffer, key }, '\n') => {
                ret.push(Node::CmakeDefine {
                    key: std::mem::take(key),
                    value: std::mem::take(buffer),
                });
                state = State::Text {
                    buffer: String::from(ch),
                };
            }
            (State::CmakeDefineValue { buffer, .. }, ch) => {
                buffer.push(ch);
            }
        }
    }

    match &mut state {
        State::Text { buffer } => {
            if !buffer.is_empty() {
                ret.push(Node::Text(std::mem::take(buffer)));
            }
        }
        _ => {
            bail!("invalid end state");
        }
    }

    Ok(ret)
}
