#[cfg(not(target_os = "windows"))]
compile_error!("Sage does not work for non-win32 systems");

use colour::{e_green, e_green_ln};
use sage::{
    cheat::{CheatHandler, Speed},
    cli::CLI,
    program::Program,
    rpc::rpc,
    util::{CVoidWrapper, Util},
};
use std::sync::Arc;
use std::thread::spawn;

const VERSION: &str = "1.17.10";
const HEADER: &str = "___________     ____   ____       ____ ___  ___ ____  
 /  ___/\\__  \\   / ___\\_/ __ \\    _/ __ \\  \\/  // __ \\ 
 \\___ \\  / __ \\_/ /_/  >  ___/    \\  ___/ >    <\\  ___/ 
/____  >(____  /\\___  / \\___  > /\\ \\___  >__/\\_ \\\\___  >
     \\/      \\//_____/      \\/  \\/     \\/      \\/    \\/\n";

pub fn main() {
    spawn(move || rpc());
    unsafe {
        let mut program = Program::new();
        program.hook();

        let mut cli = CLI::new();

        let mut cheats = CheatHandler::new();
        cheats.register(Box::new(Speed {}));
        
        e_green_ln!(HEADER);
        e_green_ln!("Developed By: Cqdet");
        e_green_ln!("Minecraft Version: {}", VERSION);
        
        loop {
            e_green!("sage> ");
            let mut cmd = String::new();
            cli.read(&mut cmd);
            Util::clean_str(&mut cmd);

            let split: Vec<&str> = cmd.split(" ").collect::<_>();

            let base_addr = program.mod_base_addr.unwrap();
            let game_id = base_addr + 0x041457C8;

            let cheat = cheats.get(*split.get(0).unwrap());
            if cheat.is_none() {
                break;
            }

            let cheat = cheat.unwrap();
            let handle = Arc::new(CVoidWrapper(program.handle.unwrap()));
            
            if cheats.threads.contains_key(cheat.name()) {
                let thread = &*cheats.threads.get(cheat.name()).unwrap();
                drop(thread);
                let thread = cheat.run(handle, game_id, 0);
                cheats.threads.insert(cheat.name(), thread);
                continue;
            }

            let thread = cheat.run(handle, game_id, 0);
            cheats.threads.insert(cheat.name(), thread);
        }
    }
}
