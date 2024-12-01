use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    property: char, // '-' means no condition
    comparison: char, // '-' means no condition
    val: usize, // 0 if no condition
    dest: String, // A / R / rule name
}
fn parse_rule(inp:&str) -> (String, Vec<Rule>) {
    // px{a<2006:qkq,m>2090:A,rfg}
    let name:String = inp.chars().take_while(|&c| c != '{').collect();
    let rules_binding:String = inp.chars()
        .skip_while(|&c| c != '{').skip(1).take_while(|&c| c != '}')
        .collect();
    let rules_str = rules_binding.split(',').collect::<Vec<&str>>();
    let mut rules:Vec<Rule> = Vec::with_capacity(rules_str.len());
    for rule in rules_str {
        if rule.contains(':') {
            rules.push(Rule {
                property: rule.chars().nth(0).unwrap(),
                comparison: rule.chars().nth(1).unwrap(),
                val: rule.chars().skip(2).take_while(|&c| c != ':')
                    .collect::<String>().parse::<usize>().unwrap(),
                dest: rule.chars().skip_while(|&c| c != ':').skip(1).collect::<String>(),
            })
        } else {
            rules.push(Rule {
                property: '-',
                comparison: '-',
                val: 0,
                dest: rule.to_string(),
            })
        }
    }
    (name, rules)
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
fn parse_part(inp:&str) -> Part {
    // {x=787,m=2655,a=1222,s=2876}
    let part_binding:String = inp.chars()
        .skip(1).take_while(|&c| c != '}').collect();
    let part_vec = part_binding.split(',').collect::<Vec<&str>>();
    fn pp(i:&str) -> usize {
        i.chars().skip(2).collect::<String>().parse::<usize>().unwrap()
    }
    // assume x m a s order
    Part {
        x: pp(&part_vec[0]),
        m: pp(&part_vec[1]),
        a: pp(&part_vec[2]),
        s: pp(&part_vec[3]),
    }
}

fn evaluate_rule(r:&Vec<Rule>, p:&Part) -> String {
    // [a<2006:qkq,m>2090:A,rfg] [x=787,m=2655,a=1222,s=2876]
    // println!("evaluating rule {:?} for part {:?}", r, p);
    for rule in r {
        if rule.val == 0 {
            // directly sends to output
            return rule.dest.clone();
        } else {
            let part_val = match rule.property {
                'x' => p.x,
                'm' => p.m,
                'a' => p.a,
                's' => p.s,
                _ => unreachable!()
            };
            match rule.comparison {
                '<' => if part_val < rule.val { return rule.dest.clone(); },
                '>' => if part_val > rule.val { return rule.dest.clone(); },
                _ => unreachable!()
            }
        }
    }
    r.last().unwrap().dest.clone()
}

pub fn y23q19q1(input:Vec<String>) -> String {

    let mut rule_map:HashMap<String,Vec<Rule>> = HashMap::new();

    for ln in &input {
        // descriptions of rules are over
        if ln.is_empty() { break; }
        // parse rule
        let (rule_name, rules) = parse_rule(ln);
        rule_map.insert(rule_name.to_string(),rules);
    }
    // println!("{:?}", rule_map);

    // process parts starting after rules
    let mut sum_of_accepted = 0;
    for ln in input.iter().skip(rule_map.len()+1) {
        let part = parse_part(ln);
        let mut next_rule = "in".to_string();
        loop {
            let res = evaluate_rule(rule_map.get(&next_rule).unwrap(), &part);
            if res == "A" {
                sum_of_accepted += part.x + part.m + part.a + part.s;
                break;
            } else if res == "R" {
                break;
            } else { next_rule = res }
        }
    }

    sum_of_accepted.to_string()
}

#[derive(Debug, Copy, Clone)]
// x0 x1 m0 m1 a0 a1 s0 s1 (0 included 1 excluded)
struct Rng(usize, usize, usize, usize, usize, usize, usize, usize);

pub fn y23q19q2(input:Vec<String>) -> String {

    let mut rule_map:HashMap<String,Vec<Rule>> = HashMap::new();

    for ln in &input {
        // descriptions of rules are over
        if ln.is_empty() { break; }
        // parse rule
        let (rule_name, rules) = parse_rule(ln);
        rule_map.insert(rule_name,rules);
    }

    let mut sum_of_accepted_ranges = 0;
    // contains both the range definition and the next rule to apply
    let mut curr_ranges:Vec<(Rng, &str)> = Vec::from([
        (Rng(1, 4001, 1, 4001, 1, 4001, 1, 4001), "in")
    ]);

    fn calc_comb(r:&Rng) -> usize {
        (r.1 - r.0) * (r.3 - r.2) * (r.5 - r.4) * (r.7 - r.6)
    }

    while let Some((rng, rule_name)) = curr_ranges.pop() {
        // println!("evaluating rule {} on rng {:?}", rule_name, rng);
        // println!("remaining ranges: {:?}", curr_ranges);
        if rule_name == "A" {
            // println!("rule {} accepts range {:?}", rule_name, rng);
            sum_of_accepted_ranges += calc_comb(&rng);
            continue;
        } else if rule_name == "R" {
            continue;
        }
        let rules = rule_map.get(rule_name).unwrap();
        //       char       char       usize    String
        //    property=a comparison=< val=2006 dest=qkq
        //    property=m comparison=> val=2090 dest=A
        //    property=- comparison=- val=0    dest=rfg
        let mut range_to_eval: Rng = rng;
        for rule in rules {
            // println!("range_to_eval {:?} for rule {:?}", range_to_eval, rule);
            if rule.val != 0 {
                let value = rule.val;
                match rule.property {
                    'x' => {
                        match rule.comparison {
                            '>' => {
                                if value < range_to_eval.0 {
                                    curr_ranges.push((range_to_eval, &rule.dest));
                                    break;
                                } else if value < range_to_eval.1 - 1 {
                                    let mut extra_range = range_to_eval;
                                    extra_range.0 = value + 1;
                                    curr_ranges.push((extra_range, &rule.dest));
                                    range_to_eval.1 = value + 1;
                                }
                            },
                            '<' => {
                                if value >= range_to_eval.1 {
                                    curr_ranges.push((range_to_eval, &rule.dest));
                                    break;
                                } else if value > range_to_eval.0 {
                                    let mut extra_range = range_to_eval;
                                    extra_range.1 = value;
                                    curr_ranges.push((extra_range, &rule.dest));
                                    range_to_eval.0 = value;
                                }
                            },
                            _ => unreachable!()
                        }
                    },
                    'm' => {
                        match rule.comparison {
                            '>' => {
                                if value < range_to_eval.2 {
                                    curr_ranges.push((range_to_eval, &rule.dest));
                                    break;
                                } else if value < range_to_eval.3 - 1 {
                                    let mut extra_range = range_to_eval;
                                    extra_range.2 = value + 1;
                                    curr_ranges.push((extra_range, &rule.dest));
                                    range_to_eval.3 = value + 1;
                                }
                            },
                            '<' => {
                                if value >= range_to_eval.3 {
                                    curr_ranges.push((range_to_eval, &rule.dest));
                                    break;
                                } else if value > range_to_eval.2 {
                                    let mut extra_range = range_to_eval;
                                    extra_range.3 = value;
                                    curr_ranges.push((extra_range, &rule.dest));
                                    range_to_eval.2 = value;
                                }
                            },
                            _ => unreachable!()
                        }                    },
                    'a' => {
                        match rule.comparison {
                            '>' => {
                                if value < range_to_eval.4 {
                                    curr_ranges.push((range_to_eval, &rule.dest));
                                    break;
                                } else if value < range_to_eval.5 - 1 {
                                    let mut extra_range = range_to_eval;
                                    extra_range.4 = value + 1;
                                    curr_ranges.push((extra_range, &rule.dest));
                                    range_to_eval.5 = value + 1;
                                }
                            },
                            '<' => {
                                if value >= range_to_eval.5 {
                                    curr_ranges.push((range_to_eval, &rule.dest));
                                    break;
                                } else if value > range_to_eval.4 {
                                    let mut extra_range = range_to_eval;
                                    extra_range.5 = value;
                                    curr_ranges.push((extra_range, &rule.dest));
                                    range_to_eval.4 = value;
                                }
                            },
                            _ => unreachable!()
                        }                    },
                    's' => {
                        match rule.comparison {
                            '>' => {
                                if value < range_to_eval.6 {
                                    curr_ranges.push((range_to_eval, &rule.dest));
                                    break;
                                } else if value < range_to_eval.7 - 1 {
                                    let mut extra_range = range_to_eval;
                                    extra_range.6 = value + 1;
                                    curr_ranges.push((extra_range, &rule.dest));
                                    range_to_eval.7 = value + 1;
                                }
                            },
                            '<' => {
                                if value >= range_to_eval.7 {
                                    curr_ranges.push((range_to_eval, &rule.dest));
                                    break;
                                } else if value > range_to_eval.6 {
                                    let mut extra_range = range_to_eval;
                                    extra_range.7 = value;
                                    curr_ranges.push((extra_range, &rule.dest));
                                    range_to_eval.6 = value;
                                }
                            },
                            _ => unreachable!()
                        }                    },
                    _ => unreachable!()
                }
            } else {
                if rule.dest == "A" {
                    // println!("rule {} accepts range {:?}", rule_name, range_to_eval);
                    sum_of_accepted_ranges += calc_comb(&range_to_eval)
                } else if rule.dest != "R" {
                    curr_ranges.push((range_to_eval, &rule.dest))
                }
            }
        }
    }

    sum_of_accepted_ranges.to_string()
}