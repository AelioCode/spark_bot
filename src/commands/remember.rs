use crate::core::input::get_input;
use crate::core::context::Context;

pub fn handle_remember(ctx: &mut Context) {
    println!("Que dois-je me souvenir ?");
    ctx.memory = get_input();
    println!("Ok, je m'en souviendrai !");
}