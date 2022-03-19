use crate::cheat::RunnableCheat;
use std::collections::HashMap;
use std::thread::JoinHandle;

pub struct CheatHandler {
    pub threads: HashMap<&'static str, JoinHandle<()>>,
    pub cheats: Vec<Box<dyn RunnableCheat>>,
}

impl CheatHandler {
    pub fn new() -> Self {
        Self {
            threads: HashMap::new(),
            cheats: vec![],
        }
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn RunnableCheat>> {
        let cheats = &self.cheats;
        let b = cheats.into_iter().find(|cheat| cheat.name() == name);
        b.clone()
    }

    pub fn register(&mut self, cheat: Box<dyn RunnableCheat>) {
        self.cheats.push(cheat)
    }
}
