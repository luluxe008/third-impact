use std::f32::consts::PI;
use std::time::Instant;

use sysinfo::{Pid, Process, System, ProcessRefreshKind, ProcessesToUpdate, MINIMUM_CPU_UPDATE_INTERVAL};

use notan::prelude::*;
use notan::draw::*;

#[derive(AppState)]
struct State{
    system: System,
    first_refresh: Instant,

    last_refresh: Instant
}

#[notan_main]
fn main() -> Result<(), String>{
    notan::init_with(setup)
    .add_config(DrawConfig)
    
    .add_config(WindowConfig{
        title: String::from("Hello"),
        transparent: true,
        
        mouse_passthrough: true,
        decorations: false,
        always_on_top: true, 
        fullscreen: true,

        ..Default::default()
    })
    
    .draw(draw)
    .update(update)
    .build()
    
    /* 
    let mut system = System::new_all();
    //system.refresh_processes(sysinfo::ProcessesToUpdate::All, false);
    system.refresh_processes_specifics(
    ProcessesToUpdate::All,
    true,
    ProcessRefreshKind::nothing().with_cpu()
    );
    std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    system.refresh_processes_specifics(
    ProcessesToUpdate::All,
    true,
    ProcessRefreshKind::nothing().with_cpu()
    );
    
    let processes = system.processes();
    let mut vec = processes.iter()
                            //.filter(|process| process.1.name().display().to_string() == "firefox.exe")
                            .collect::<Vec<(&Pid, &Process)>>();
    vec.sort_by(|a, b| a.1.cpu_usage().total_cmp(&b.1.cpu_usage()));
    for process in &vec{
        println!("{} [{}] {} {}%", process.0, process.1.parent().unwrap_or(Pid::from_u32(255)), process.1.name().display(), process.1.cpu_usage())
    }
    match vec.choose(&mut rand::rng()){
        Some(pro) => {
            println!();
            println!("on va taper dans {}", pro.1.name().display())
        }
        None => panic!("wtf")
        }
    */
}


fn setup() -> State{
    let mut system = System::new();
    system.refresh_cpu_all();

    State{
        system: system,
        first_refresh: Instant::now(),
        last_refresh: Instant::now(),
    }
}

fn draw_emergency(draw: &mut Draw, coord: (f32, f32)){
    draw.polygon(6, 100.0f32).position(coord.0, coord.1).rotate_from(coord, PI/6f32).color(Color::RED);

}

fn draw(gfx: &mut Graphics){
    let mut draw = gfx.create_draw();
    draw.clear(Color::TRANSPARENT);
    
    draw_emergency(&mut draw, (500f32, 500f32));
    gfx.render(&draw);
}


fn update(app: &mut App, state: &mut State) {
    if state.last_refresh+MINIMUM_CPU_UPDATE_INTERVAL >= Instant::now(){
        state.system.refresh_cpu_all();
        state.last_refresh = Instant::now();
    }
    if state.first_refresh+MINIMUM_CPU_UPDATE_INTERVAL>= Instant::now(){
    }

    if app.keyboard.is_down(KeyCode::Escape){
        app.exit();
    }
}