use std::f32::consts::PI;
use std::time::Duration;
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

fn draw(gfx: &mut Graphics, state: &mut State){
    let mut draw = gfx.create_draw();

    draw.clear(Color::TRANSPARENT);
    
    if state.first_refresh+MINIMUM_CPU_UPDATE_INTERVAL <= Instant::now(){
        let cpus = state.system.cpus();
        dbg!(cpus.len());
        let x = 500f32;
        let y = 100f32;
        let offset_x = 300f32;
        let offset_y = 300f32;
        let mut count_x = 0f32;
        let mut count_y = 0f32;
        for cpu in cpus{
            println!("{}", cpu.name());
            draw_emergency(&mut draw, (x+count_x*offset_x, y+count_y*offset_y));
            count_x += 1.0;
            if count_x == 3.0{
                count_x = 0.0;
                count_y += 1.0;
            }
        }
    
    }
    gfx.render(&draw);
}


fn update(app: &mut App, state: &mut State) {
    if state.last_refresh+Duration::from_secs(1) <= Instant::now(){
        state.system.refresh_cpu_all();
        state.last_refresh = Instant::now();
        println!("update");
    }
    

    if app.keyboard.is_down(KeyCode::Escape){
        app.exit();
    }
}