use glutin::event_loop::EventLoop;
use glutin::platform::unix::HeadlessContextExt;
use glutin::platform::ContextTraitExt;
use glutin::ContextBuilder;
// use glutin::dpi::PhysicalSize;
use core::ops::Deref;

fn main() {
    let ctb = ContextBuilder::new();
    // let elt = EventLoop::new().deref();
    let elp = EventLoop::new();
    // let ps = PhysicalSize::new(800, 600);
    // let ctx = ctb.build_surfaceless(elt);
    let ctx_ = ctb.build_surfaceless(elp.deref());

    match ctx_ {
        Ok(ctx) => {
            unsafe {
            	let dsp = ctx.get_egl_display();
            	match dsp {
                	Some(_) => println!("It worked!"),
                	None => println!("Created context, can't get display.")
            	}
            }
        },
        Err(_) => println!("NOPE")
    }
}
