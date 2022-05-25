use crate::syntax::ast::*;
use crate::syntax::tokens::*;

pub fn parser(tokens: &[Token], v: &[char]) -> Nodes {
    use TokenType::*;

    let mut nodes: Nodes = Vec::new();
    let mut info = ParseInfo::new(tokens.len());
    let mut buffer = String::new();
    //println!("{:?}", parse_body::<ItemIngredients>(tokens, &mut info, v));

    while info.ptr <= info.end {
        match tokens[info.ptr].t {
            At => {
                if buffer.is_empty() {
                } else {
                    nodes.push(
                        ItemText {
                            body: buffer.clone(),
                        }
                        .as_node(),
                    );
                    buffer.clear();
                }
                // yyy @xxx{111.222%g} yyy
                if let Some(item) = parse_body::<ItemIngredients>(tokens, &mut info, v) {
                    nodes.push(item.as_node());
                } else {
                }
            }

            Pound => {
                if buffer.is_empty() {
                } else {
                    nodes.push(
                        ItemText {
                            body: buffer.clone(),
                        }
                        .as_node(),
                    );
                    buffer.clear();
                }
                // yyy #xxx{111.222%g} yyy
                if let Some(item) = parse_body::<ItemCookware>(tokens, &mut info, v) {
                    nodes.push(item.as_node());
                } else {
                }
            }

            Tilde => {
                if buffer.is_empty() {
                } else {
                    nodes.push(
                        ItemText {
                            body: buffer.clone(),
                        }
                        .as_node(),
                    );
                    buffer.clear();
                }
                // yyy ~xxx{111.222%g} yyy
                if let Some(item) = parse_body::<ItemTimer>(tokens, &mut info, v) {
                    nodes.push(item.as_node());
                } else {
                }
            }

            Minus => {
                if let Some(pos) = try_parse_minus(tokens, &mut info) {
                    nodes.push(
                        ItemComments {
                            body: slice_to_string(&v[pos.0..=pos.1]),
                        }
                        .as_node(),
                    );
                } else {
                    // End1
                    // a -bbb
                    buffer.push('-');
                    info.next();
                }
            }

            OpenBracket => {
                if let Some(pos) = try_parse_openbracket(tokens, &mut info) {
                    nodes.push(
                        ItemComments {
                            body: slice_to_string(&v[pos.0..=pos.1]),
                        }
                        .as_node(),
                    );
                } else {
                    buffer.push('[');
                    info.next();
                }
            }

            Text => {
                let pos = tokens[info.ptr].p;
                buffer.push_str(slice_to_string(&v[pos.start..=pos.end]).as_str());
            }

            WhiteSpace => {
                buffer.push(' ');
            }

            NewLine => {
                if buffer.is_empty() {
                } else {
                    nodes.push(
                        ItemText {
                            body: buffer.clone(),
                        }
                        .as_node(),
                    );
                    buffer.clear();
                }

                nodes.push(ItemSign::NewLine.as_node());
            }

            _ => {}
        }
        info.next();
    }

    nodes
}

pub fn parse_metadata() {}
pub fn parse_timer() {}
pub fn parse_comment() {}

pub fn parse_body<T>(tokens: &[Token], info: &mut ParseInfo, v: &[char]) -> Option<T>
where
    T: Change + std::fmt::Debug + Sized,
{
    let mut quantity_pos = Pos::new();
    let mut item = <T as Change>::new();

    item.set_full_start(tokens[info.ptr].p.start);

    // @xxx
    if tokens[info.peek()].t == TokenType::Text {
        item.start(tokens[info.peek()].p.start);
        //println!("{:?}", item.body);
        info.next();
        // @xxx{
        if try_parse_openbrace(tokens, info, &mut item).is_ok() {
            if try_parse_float(tokens, info, &mut quantity_pos).is_ok() {
                if try_parse_percent(tokens, info, &mut item, &mut quantity_pos, v).is_ok() {
                    if tokens[info.peek()].t == TokenType::Text {
                        item.set_unit_start(tokens[info.peek()].p.start);
                        info.next(); // @xxx {111.222%g
                        if try_parse_closebrace(tokens, info, &mut item).is_ok() {
                            return Some(item); // Done
                        }
                    }
                }
            } else if tokens[info.peek()].t == TokenType::Percent {
                return None; // End3
            } else if try_parse_closebrace(tokens, info, &mut item).is_ok() {
                // yyy @xxx{}
                return Some(item); // Done
            } else {
                // yyy @xxx {
                return None;
            }
        } else if tokens[info.peek()].t == TokenType::Text
            || tokens[info.peek()].t == TokenType::WhiteSpace
        {
            item.start(tokens[info.ptr].p.start);
            info.next(); // @xxx

            while tokens[info.peek()].t == TokenType::Text
                || tokens[info.peek()].t == TokenType::WhiteSpace
            {
                info.next(); // @xxx xxx
            }

            if try_parse_openbrace(tokens, info, &mut item).is_ok() {
                if try_parse_float(tokens, info, &mut quantity_pos).is_ok() {
                    if tokens[info.peek()].t == TokenType::Percent {
                        if tokens[info.peek()].t == TokenType::Text {
                            item.set_unit_start(tokens[info.peek()].p.start);
                            info.next(); // @xxx xxx xxx {111.222%g
                            if try_parse_closebrace(tokens, info, &mut item).is_ok() {
                                item.set_unit_end(tokens[info.peek()].p.end);
                                return Some(item);
                                // Done
                            }
                        }
                    }
                } else if tokens[info.peek()].t == TokenType::Percent {
                    // eprintln!("Err");
                    return None; // End3
                } else if try_parse_closebrace(tokens, info, &mut item).is_ok() {
                    // yyy @xxx{}
                    return Some(item); // Done
                } else {
                    // yyy @xxx {
                    return None;
                }
            } else {
                // yyy @xxx yyy
                return None; // End2
            }
        } else {
            // yyy @xxx yyy
            return None; // End2
        }
    } else if tokens[info.peek()].t == TokenType::WhiteSpace {
        // yyy @xxx yyy
        return Some(item); // Done
    } else {
        return None; // End1
    };

    Some(item)
}

pub fn try_parse_float(
    tokens: &[Token],
    info: &mut ParseInfo,
    quantity_pos: &mut Pos,
) -> Result<(), ()> {
    if tokens[info.peek()].t == TokenType::Float {
        quantity_pos.start = tokens[info.peek()].p.start;
        info.next(); // @xxx {111.222
        Ok(())
    } else {
        Err(())
    }
}

pub fn try_parse_percent(
    tokens: &[Token],
    info: &mut ParseInfo,
    item: &mut impl Change,
    quantity_pos: &mut Pos,
    v: &[char],
) -> Result<(), ()> {
    if tokens[info.peek()].t == TokenType::Percent {
        // tokens[info.peek()] is '%'
        // tokens[info.ptr ] is Float
        quantity_pos.end = tokens[info.ptr].p.end;
        //println!("{:?}", quantity_pos);
        item.set_quantity(
            slice_to_string(&v[quantity_pos.start..=quantity_pos.end])
                .parse::<f64>()
                .expect(""),
        );
        info.next(); // @xxx {111.222%
        Ok(())
    } else {
        Err(())
    }
}

pub fn try_parse_closebrace(
    tokens: &[Token],
    info: &mut ParseInfo,
    item: &mut impl Change,
) -> Result<(), ()> {
    if tokens[info.peek()].t == TokenType::CloseBrace {
        item.set_unit_end(tokens[info.peek()].p.end - 1);
        item.set_full_end(tokens[info.peek()].p.end - 1);
        info.next(); // @xxx {111.222%g}
        Ok(())
    } else {
        Err(())
    }
}

pub fn try_parse_openbrace(
    tokens: &[Token],
    info: &mut ParseInfo,
    item: &mut impl Change,
) -> Result<(), ()> {
    if tokens[info.peek()].t == TokenType::OpenBrace {
        item.end(tokens[info.ptr].p.end);
        info.next(); // @xxx xxx xxx {
        Ok(())
    } else {
        Err(())
    }
}

pub fn try_parse_minus(tokens: &[Token], info: &mut ParseInfo) -> Option<(usize, usize)> {
    // aaa -- xxx yyy

    let start;
    let end;
    if info.has_next() && tokens[info.ptr].t == TokenType::Minus {
        info.next();
        start = info.ptr;
        while info.has_next() && tokens[info.peek()].t != TokenType::NewLine {
            info.next();
        }
        end = info.ptr;
        Some((start, end))
    } else {
        None
    }
}

pub fn try_parse_openbracket(tokens: &[Token], info: &mut ParseInfo) -> Option<(usize, usize)> {
    let start;
    let end;
    // aaa [-xxx yyy-] aaa
    if info.has_next() && tokens[info.peek()].t == TokenType::Minus {
        info.next();
        start = info.ptr;
        'l1: loop {
            if info.has_next() && tokens[info.peek()].t == TokenType::Minus {
                info.next();
                if info.has_next() && tokens[info.peek()].t == TokenType::CloseBracket {
                    end = info.ptr - 1;
                    info.next();
                    break 'l1;
                }
            } else {
                info.next();
            }
        }
        Some((start, end))
    } else {
        None
    }
}

pub fn slice_to_string(s: &[char]) -> String {
    let mut res = String::new();

    for f in s.iter() {
        res.push(*f);
    }

    res
}

// https://users.rust-lang.org/t/cannot-figure-out-why-i-cannot-access-a-structs-field/29775/7
