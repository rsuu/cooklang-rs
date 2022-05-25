use crate::syntax::ast::*;
use crate::syntax::parser::slice_to_string;
use crate::syntax::tokens;
use std::collections::HashMap;

pub fn parse_nodes(nodes: Nodes, v: &[char]) {
    let mut info = tokens::ParseInfo::new(nodes.len());
    let mut body = String::with_capacity(v.len() * 4);
    body.push_str("Steps:\n");
    let mut ig_map: HashMap<String, usize> = HashMap::new();
    let mut cw_map: HashMap<String, usize> = HashMap::new();

    let mut step = 1;
    let mut buf = String::with_capacity(300);
    let mut step_buf = String::with_capacity(100);
    let mut step_map: HashMap<String, usize> = HashMap::with_capacity(10);
    // step_map <name,quantity,unit>

    while info.ptr <= info.end {
        match &nodes[info.ptr] {
            Node::ItemText(t) => {
                buf.push_str(t.body.as_str());
                // info.next();
                // parse_body(&nodes, &mut info, &mut step, &mut body, &mut ig_map, v);
            }

            Node::ItemIngredients(ig) => {
                let ig_name = slice_to_string(&v[ig.body.start..=ig.body.end]);
                // println!("{:?}", ig.body);
                buf.push_str(format!("{}{}", '@', &ig_name).as_str());

                if step_map.contains_key(&ig_name) {
                    *step_map.entry(ig_name).or_insert(0) += ig.quantity as usize;
                } else {
                    step_map.insert(ig_name, ig.quantity as usize);
                }
            }

            Node::ItemSign(ItemSign::NewLine) => {
                if step_map.is_empty() {
                    step_buf.push_str("[-]");
                } else {
                    step_buf.push_str("[");

                    for (k, v) in step_map.iter() {
                        step_buf.push_str(format!("{}: {} | ", k, v).as_str());
                    }

                    step_buf.push(']');
                    // DONE
                    // step_buf

                    // ig_map
                    if ig_map.is_empty() {
                        merge_mut(&mut ig_map, step_map.clone());
                    } else {
                        //println!("ig: {:?}", ig_map);
                        //println!("{:?}", step_map);

                        for f in step_map.clone().into_iter() {
                            if ig_map.contains_key(f.0.as_str()) {
                                *ig_map.entry(f.0).or_insert(0) += f.1;
                            } else {
                                let mut map: HashMap<String, usize> = HashMap::from([f]);
                                merge_mut(&mut ig_map, map);
                            }
                        }
                    }
                }

                if buf.is_empty() {
                    step_buf.clear();
                } else {
                    body.push_str(
                        format!("    {}. {}\n        {}\n", step, &buf, &step_buf).as_str(),
                    );
                    step += 1;

                    step_map.clear();
                    buf.clear();
                    step_buf.clear();
                }
            }

            Node::ItemCookware(cw) => {
                let cw_name = slice_to_string(&v[cw.body.start..=cw.body.end]);
                // println!("{:?}", cw.body);
                buf.push_str(format!("{}{}", '#', &cw_name).as_str());
            }

            Node::ItemTimer(ti) => {
                let ti_name = slice_to_string(&v[ti.body.start..=ti.body.end]);
                // println!("{:?}", ti.body);
                buf.push_str(format!("{}{}", '~', &ti_name).as_str());
            }

            _ => {}
        }
        info.next();
    }

    println!("{:#?}", ig_map);
    println!("{}", body);
}

fn find_value<K, V>(map: &HashMap<K, V>, search: &str) -> Option<V>
where
    K: PartialEq<str> + ?Copy + Clone, // String
    V: PartialEq + Copy,               // usize
{
    // https://stackoverflow.com/questions/59401720/how-do-i-find-the-key-for-a-value-in-a-hashmap
    map.iter()
        .find_map(|(name, &val)| if name == search { Some(val) } else { None })
}

fn merge_mut(org: &mut HashMap<String, usize>, map: HashMap<String, usize>) {
    org.extend(map);
}

/*
Ingredients:
    雪梨                            130 g
    雪梨 aaa                        11 gsses

Steps:
     1. yyy 雪梨 yyy 雪梨 awd
        [雪梨: 120 g]
     2. yyy 雪梨 aaa yyy xxx 雪梨 awd
        [雪梨: 2.7 g; 雪梨 aaa: 11 gsses]
     3. awd wdawd
        [–]
*/
