use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day19_test.txt"), 19114);
    dbg!(part1("assets/day19.txt"));
    assert_eq!(part2("assets/day19_test.txt"), 167409079868000);
    dbg!(part2("assets/day19.txt"));
}

fn part2(file: &str) -> u64 {
    let (_, workflows) = parse_file(file);
    let item_range = ItemRange {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
    };
    workflows
        .get("in")
        .unwrap()
        .num_accepted(item_range, &workflows)
}

fn part1(file: &str) -> u64 {
    let (mut items, workflows) = parse_file(file);
    items.retain(|i| i.accepted(&workflows));
    items.iter().map(|i| i.x + i.m + i.a + i.s).sum::<u32>() as u64
}

fn parse_file(file: &str) -> (Vec<Item>, HashMap<String, Action>) {
    let file = read_file(file);
    let mut workflows = HashMap::new();
    let mut items = Vec::new();
    let mut add_items = false;

    for line in file.lines() {
        if line.is_empty() {
            add_items = true;
            continue;
        }
        if add_items {
            let (x, m, a, s): (u32, u32, u32, u32);
            scan!(line.bytes() => "[x={},m={},a={},s={}]", x, m, a, s);
            items.push(Item { x, m, a, s });
        } else {
            let (w, a): (String, String);
            scan!(line.bytes() => "{}[{}]", w, a);
            let action = {
                a.split(',')
                    .rev()
                    .fold(None, |a, b| {
                        let action = |string| match string {
                            "A" => Action::Accept,
                            "R" => Action::Reject,
                            x => Action::Workflow(x.to_string()),
                        };
                        if let Some(els) = a {
                            let (fcv, t) = b.split_once(':').unwrap();
                            let (f, cv) = fcv.split_at(1);
                            let (c, v) = cv.split_at(1);
                            let field = match f {
                                "x" => Field::X,
                                "m" => Field::M,
                                "a" => Field::A,
                                "s" => Field::S,
                                _ => unreachable!(),
                            };
                            let comp = match c {
                                ">" => Comp::Gt,
                                "<" => Comp::Lt,
                                _ => unreachable!(),
                            };
                            let val = v.parse::<u32>().unwrap();
                            let then = action(t);
                            Some(Action::Condition(Box::new(Condition {
                                field,
                                comp,
                                val,
                                then,
                                els,
                            })))
                        } else {
                            Some(action(b))
                        }
                    })
                    .unwrap()
            };
            workflows.insert(w, action);
        }
    }

    (items, workflows)
}

#[derive(Clone, Debug)]
struct ItemRange {
    x: Range<u32>,
    m: Range<u32>,
    a: Range<u32>,
    s: Range<u32>,
}

impl ItemRange {
    fn num(&self) -> u64 {
        (self.x.len() * self.m.len() * self.a.len() * self.s.len()) as u64
    }
}

#[derive(Copy, Clone, Debug)]
struct Item {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Item {
    fn accepted(&self, workflows: &HashMap<String, Action>) -> bool {
        match self.eval(workflows, "in") {
            Action::Accept => true,
            Action::Reject => false,
            _ => unreachable!(),
        }
    }

    fn eval<'a>(&'a self, workflows: &'a HashMap<String, Action>, workflow: &'a str) -> &'a Action {
        match workflows.get(workflow).unwrap() {
            x @ Action::Accept | x @ Action::Reject => x,
            Action::Condition(cond) => {
                let x = cond.eval(self);
                if let Action::Workflow(work) = x {
                    self.eval(workflows, work)
                } else {
                    x
                }
            }
            Action::Workflow(work) => self.eval(workflows, work),
        }
    }
}

enum Action {
    Condition(Box<Condition>),
    Workflow(String),
    Accept,
    Reject,
}

impl Action {
    fn num_accepted(&self, item_range: ItemRange, workflows: &HashMap<String, Action>) -> u64 {
        match self {
            Action::Accept => item_range.num(),
            Action::Reject => 0,
            Action::Condition(cond) => {
                let (a, b) = match (&cond.field, &cond.comp) {
                    (Field::X, Comp::Gt) => (
                        if cond.val >= item_range.x.end {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.x = ir.x.start.max(cond.val + 1)..ir.x.end;
                            cond.then.num_accepted(ir, workflows)
                        },
                        if cond.val <= item_range.x.start {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.x = ir.x.start..ir.x.end.min(cond.val + 1);
                            cond.els.num_accepted(ir, workflows)
                        },
                    ),
                    (Field::M, Comp::Gt) => (
                        if cond.val >= item_range.m.end {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.m = ir.m.start.max(cond.val + 1)..ir.m.end;
                            cond.then.num_accepted(ir, workflows)
                        },
                        if cond.val <= item_range.m.start {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.m = ir.m.start..ir.m.end.min(cond.val + 1);
                            cond.els.num_accepted(ir, workflows)
                        },
                    ),
                    (Field::A, Comp::Gt) => (
                        if cond.val >= item_range.a.end {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.a = ir.a.start.max(cond.val + 1)..ir.a.end;
                            cond.then.num_accepted(ir, workflows)
                        },
                        if cond.val <= item_range.a.start {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.a = ir.a.start..ir.a.end.min(cond.val + 1);
                            cond.els.num_accepted(ir, workflows)
                        },
                    ),
                    (Field::S, Comp::Gt) => (
                        if cond.val >= item_range.s.end {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.s = ir.s.start.max(cond.val + 1)..ir.s.end;
                            cond.then.num_accepted(ir, workflows)
                        },
                        if cond.val <= item_range.s.start {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.s = ir.s.start..ir.s.end.min(cond.val + 1);
                            cond.els.num_accepted(ir, workflows)
                        },
                    ),
                    (Field::X, Comp::Lt) => (
                        if cond.val <= item_range.x.start {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.x = ir.x.start..ir.x.end.min(cond.val);
                            cond.then.num_accepted(ir, workflows)
                        },
                        if cond.val >= item_range.x.end {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.x = cond.val.max(ir.x.start)..ir.x.end;
                            cond.els.num_accepted(ir, workflows)
                        },
                    ),
                    (Field::M, Comp::Lt) => (
                        if cond.val <= item_range.m.start {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.m = ir.m.start..ir.m.end.min(cond.val);
                            cond.then.num_accepted(ir, workflows)
                        },
                        if cond.val >= item_range.m.end {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.m = cond.val.max(ir.m.start)..ir.m.end;
                            cond.els.num_accepted(ir, workflows)
                        },
                    ),
                    (Field::A, Comp::Lt) => (
                        if cond.val <= item_range.a.start {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.a = ir.a.start..ir.a.end.min(cond.val);
                            cond.then.num_accepted(ir, workflows)
                        },
                        if cond.val >= item_range.a.end {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.a = cond.val.max(ir.a.start)..ir.a.end;
                            cond.els.num_accepted(ir, workflows)
                        },
                    ),
                    (Field::S, Comp::Lt) => (
                        if cond.val <= item_range.s.start {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.s = ir.s.start..ir.s.end.min(cond.val);
                            cond.then.num_accepted(ir, workflows)
                        },
                        if cond.val >= item_range.s.end {
                            0
                        } else {
                            let mut ir = item_range.clone();
                            ir.s = cond.val.max(ir.s.start)..ir.s.end;
                            cond.els.num_accepted(ir, workflows)
                        },
                    ),
                };
                a + b
            }
            Action::Workflow(work) => workflows
                .get(work)
                .unwrap()
                .num_accepted(item_range, workflows),
        }
    }
}

impl Condition {
    fn eval<'a>(&'a self, item: &'a Item) -> &'a Action {
        fn eval<'a>(cond: &'a Condition, item: &'a Item) -> &'a Action {
            let val = match cond.field {
                Field::X => item.x,
                Field::M => item.m,
                Field::A => item.a,
                Field::S => item.s,
            };
            let passes = match cond.comp {
                Comp::Gt => val > cond.val,
                Comp::Lt => val < cond.val,
            };
            if passes {
                &cond.then
            } else {
                &cond.els
            }
        }

        let action = eval(self, item);
        if let Action::Condition(cond) = action {
            cond.eval(item)
        } else {
            action
        }
    }
}

struct Condition {
    field: Field,
    comp: Comp,
    val: u32,
    then: Action,
    els: Action,
}

#[derive(Copy, Clone, Debug)]
enum Comp {
    Gt,
    Lt,
}

#[derive(Copy, Clone, Debug)]
enum Field {
    X,
    M,
    A,
    S,
}
