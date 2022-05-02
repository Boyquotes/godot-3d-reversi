mod bitboard;
mod node;

pub use {
    bitboard::{BitBoard, Color},
    node::{Node, Nodes},
    std::io,
    std::time::{Duration, Instant},
};

const PHASE1_DEPTH: i32 = 10;
const PHASE2_DEPTH: i32 = 20;

const MAX_I32: i32 = i32::MAX;
const MIN_I32: i32 = i32::MIN;

pub struct AI {
    color: Color,
    opp: Color,
    phase: i32,
    depth: i32,
    reached_depth: i32,
    node: i32,
    level: i32,
}

impl AI {
    pub fn new(color: Color, lv: i32) -> AI {
        let ai = AI {
            color: color,
            opp: color.reverse(),
            phase: 1,
            reached_depth: 0,
            depth: 0,
            node: 0,
            level: lv,
        };
        ai
    }

    pub fn next_move(&mut self, input: String) -> String {
        let bd = BitBoard::new(input);
        self.set_phase(&bd);
        self.set_depth();

        self.node = 0;

        let now = Instant::now();
        let best = self.alpha_beta_helper(&bd, self.depth);
        let elapsed = now.elapsed();

        best.to_string() + " " + &self.print_msg(best, elapsed)
    }

    fn set_phase(&mut self, bd: &BitBoard) {
        let empty = bd.empty_count();
        let phase2 = PHASE2_DEPTH + (self.level - 4) * 3;
        if empty > phase2 {
            self.phase = 1;
        } else {
            self.phase = 2;
        }
    }

    fn print_msg(&self, best: Node, t: Duration) -> String {
        match self.phase {
            1 => format!(
                "{{ depth: {:2}, value: {:+.2}, nodes: {}, time: {:.4}s, NPS: {:.0} }}",
                self.reached_depth,
                best.value as f64 * ((bitboard::SIZE * bitboard::SIZE) as f64) / 1476.0,
                self.node,
                t.as_secs_f64(),
                self.node as f64 / t.as_secs_f64(),
            ),
            2 => format!(
                "{{ depth: {:2}, value: {:+}, nodes: {}, time: {:.4}s, NPS: {:.0} }}",
                self.reached_depth,
                best.value,
                self.node,
                t.as_secs_f64(),
                self.node as f64 / t.as_secs_f64(),
            ),
            _ => panic!("unknown phase"),
        }
    }

    fn set_depth(&mut self) {
        if self.phase == 1 {
            self.depth = PHASE1_DEPTH + (self.level - 4) * 2;
            if self.depth <= 0 {
                self.depth = 1;
            }
        } else {
            self.depth = MAX_I32;
        }
    }

    fn heuristic(&self, bd: &BitBoard) -> i32 {
        if self.phase == 1 {
            bd.eval(self.color)
        } else {
            bd.count(self.color) - bd.count(self.opp)
        }
    }

    fn sort_valid_nodes(&self, bd: &BitBoard, cl: Color, valid: u64) -> Nodes {
        let mut all: Nodes = Nodes::new();
        if self.phase == 1 {
            for loc in 0..bitboard::SIZE * bitboard::SIZE {
                if (1u64 << loc) & valid != 0 {
                    let mut tmp = bd.copy();
                    tmp.put(loc, cl);
                    all.push(Node::new(loc, tmp.eval(cl)));
                }
            }
            all.sort_desc();
        } else {
            let op = cl.reverse();
            for loc in 0..bitboard::SIZE * bitboard::SIZE {
                if (1u64 << loc) & valid != 0 {
                    let mut tmp = bd.copy();
                    tmp.put(loc, cl);
                    all.push(Node::new(loc, tmp.mobility(op)));
                }
            }
            all.sort_asc();
        }
        all
    }

    #[allow(dead_code)]
    fn sorted_valid_nodes(&self, bd: &BitBoard, cl: Color) -> Nodes {
        let mut all: Nodes = Nodes::new();
        let all_valid: u64 = bd.all_valid_loc(cl);
        if self.phase == 1 {
            for loc in 0..bitboard::SIZE * bitboard::SIZE {
                if (1u64 << loc) & all_valid != 0 {
                    let mut tmp = bd.copy();
                    tmp.put(loc, cl);
                    all.push(Node::new(loc, tmp.eval(cl)));
                }
            }
            all.sort_desc();
        } else {
            let op = cl.reverse();
            for loc in 0..bitboard::SIZE * bitboard::SIZE {
                if (1u64 << loc) & all_valid != 0 {
                    let mut tmp = bd.copy();
                    tmp.put(loc, cl);
                    all.push(Node::new(loc, tmp.mobility(op)));
                }
            }
            all.sort_asc();
        }
        all
    }

    fn alpha_beta_helper(&mut self, bd: &BitBoard, depth: i32) -> Node {
        self.alpha_beta(bd, depth, MIN_I32, MAX_I32, true)
    }

    fn alpha_beta(
        &mut self,
        bd: &BitBoard,
        depth: i32,
        mut alpha: i32,
        mut beta: i32,
        max_layer: bool,
    ) -> Node {
        self.node += 1;

        if depth == 0 {
            self.reached_depth = self.depth;
            let v = self.heuristic(bd);
            return Node::new(-1, v);
        }

        let self_valid = bd.all_valid_loc(self.color);
        let opp_valid = bd.all_valid_loc(self.opp);

        // // game is over
        if self_valid == 0 && opp_valid == 0 {
            self.reached_depth = self.depth - depth;
            let v = self.heuristic(bd);
            return Node::new(-1, v);
        }
        // if bd.is_over() {
        //     self.reached_depth = self.depth - depth;
        //     let v = self.heuristic(bd);
        //     return Node::new(-1, v);
        // }

        if max_layer {
            let mut max_value = MIN_I32;
            let mut best_node = Node::new(-1, max_value);

            let ai_valid = self.sort_valid_nodes(&bd, self.color, self_valid);
            // let ai_valid = self.sorted_valid_nodes(&bd, self.color);
            if ai_valid.size() == 0 {
                return self.alpha_beta(bd, depth, alpha, beta, false);
            }

            for n in ai_valid.nodes {
                let mut tmp = bd.copy();
                tmp.put(n.loc, self.color);
                let eval = self.alpha_beta(&tmp, depth - 1, alpha, beta, false).value;

                if eval > max_value {
                    max_value = eval;
                    best_node = n;
                }
                if max_value > alpha {
                    alpha = max_value;
                }

                if beta <= alpha {
                    break;
                }
            }

            return Node::new(best_node.loc, max_value);
        } else {
            let mut min_value = MAX_I32;
            let mut best_node = Node::new(-1, min_value);

            let op_valid = self.sort_valid_nodes(&bd, self.opp, opp_valid);
            // let op_valid = self.sorted_valid_nodes(&bd, self.opp);
            if op_valid.size() == 0 {
                return self.alpha_beta(bd, depth, alpha, beta, true);
            }

            for n in op_valid.nodes {
                let mut tmp = bd.copy();
                tmp.put(n.loc, self.opp);
                let eval = self.alpha_beta(&tmp, depth - 1, alpha, beta, true).value;

                if eval < min_value {
                    min_value = eval;
                    best_node = n;
                }
                if min_value < beta {
                    beta = min_value;
                }

                if beta <= alpha {
                    break;
                }
            }

            return Node::new(best_node.loc, min_value);
        }
    }
}

pub fn ai_run_once(board: String, color: i32, strength: i32) -> String {
    let level = match strength {
        0 => 0,
        1 => 2,
        2 => 4,
        _ => panic!("unknwn level"),
    };
    let mut ai = match color {
        1 => AI::new(Color::Black, level),
        2 => AI::new(Color::White, level),
        _ => panic!("error"),
    };
    return ai.next_move(board);
}
