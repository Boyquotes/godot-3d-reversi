use {gdnative::prelude::*, reversi_ai, std::sync::mpsc, std::thread};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct OthelloAI {
    sender: mpsc::Sender<std::string::String>,
    receiver: mpsc::Receiver<std::string::String>,
}

#[methods]
impl OthelloAI {
    fn new(_owner: &Node) -> Self {
        let (tx, rx) = mpsc::channel();
        OthelloAI {
            sender: tx,
            receiver: rx,
        }
    }

    #[export]
    fn run(&self, _owner: &Node, board: String, color: String, strength: String) {
        let send = self.sender.clone();
        thread::spawn(move || {
            let color = match color.to_lowercase().as_str() {
                "black" => 1,
                "white" => 2,
                _ => 2,
            };
            let strength = match strength.to_lowercase().as_str() {
                "weak" => 0,
                "medium" => 1,
                _ => 2,
            };
            let res = reversi_ai::ai_run_once(board, color, strength);
            send.send(res).unwrap();
        });
    }

    #[export]
    fn done(&self, _owner: &Node) -> String {
        let data = self.receiver.try_recv();
        if data.is_ok() {
            data.unwrap()
        } else {
            String::from("")
        }
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<OthelloAI>();
}

godot_init!(init);
