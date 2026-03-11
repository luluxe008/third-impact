use std::f32::consts::PI;
use std::time::Duration;
use std::time::Instant;
use std::env;


use hexing::HexRing;
use sysinfo::{Pid, Process, System, ProcessRefreshKind, ProcessesToUpdate, MINIMUM_CPU_UPDATE_INTERVAL};
use hexing::HexPosition;

use notan::prelude::*;
use notan::draw::*;

const ANGEL_NAMES: &'static str = include_str!("../angel_name.txt");

#[derive(AppState)]
struct State{
    system: System,
    first_refresh: Instant,
    count: u32,
    font: Font,
    names: Vec<&'static str>,

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


fn setup(app: &mut App, gfx: &mut Graphics) -> State{
    if app.window().size() != (1920, 1080){
        panic!("Wrong screen size");
    }
    let mut system = System::new();
    system.refresh_cpu_all();

    let names = ANGEL_NAMES.split("\n").collect();

    State{
        system: system,
        first_refresh: Instant::now(),
        last_refresh: Instant::now(),
        count: 0,
        font: gfx.create_font(include_bytes!("../NimbusRomNo9L-Reg.otf")).unwrap(),
        names
    }
}

fn draw_emergency(draw: &mut Draw, coord: (f32, f32), font: &Font){
    draw.polygon(6, 30.0f32).rotate(PI/6.0).translate(coord.0 * 50.0 , coord.1 *50.0 ).color(Color::RED);

    //draw.text(font, name).size(30f32).position(coord.0, coord.1).color(Color::BLACK);
}

fn draw(gfx: &mut Graphics, state: &mut State){
    let mut draw = gfx.create_draw();

    draw.clear(Color::TRANSPARENT);
    let center: HexPosition<i32> = HexPosition::new(6,6 );
    let mut hex: Vec<HexPosition<i32>> = Vec::new();
    hex.push(center);
    for r in 1..19{
        hex.append(&mut center.ring(r).collect());
    }
    if state.first_refresh+MINIMUM_CPU_UPDATE_INTERVAL <= Instant::now(){

        for pos in hex.get(0..state.count as usize).unwrap(){
            draw_emergency(&mut draw, pos.to_pixel_coordinates(), &state.font);

        }

    }
    gfx.render(&draw);
}


fn update(app: &mut App, state: &mut State) {
    if state.last_refresh+Duration::from_millis(10) < Instant::now(){
        state.count += 1;
        state.last_refresh = Instant::now()
    }
    

    if app.keyboard.is_down(KeyCode::Escape){
        app.exit();
    }
}