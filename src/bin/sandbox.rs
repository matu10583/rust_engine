use rust_engine::core;
use rust_engine::ecs;
use rust_engine::{DiContainer, Events, Time, TimeState};

#[derive(Debug, Clone, Copy)]
struct Position(f32, f32);
// impl Component for Position {}

#[derive(Debug, Clone, Copy)]
struct Velocity(f32, f32);
// impl Component for Velocity {}

#[derive(Debug, Clone)]
struct Foo(u32);

fn main() {
    test_ecs();
    test_events();
    test_time_in_di();
    test_app();
}

fn test_app() {
    let _ = env_logger::try_init();

    let mut app = core::App::new();

    app.add_system(rust_engine::core::Stage::Startup, |di, world| {
        println!("Startup system running");
        di.insert(String::from("Hello from DI Container"));
        let e = world.spawn(Position(0.0, 0.0));
        world.insert(e, Velocity(1.0, 1.0));
    });

    app.add_system(rust_engine::core::Stage::Update, |di, world| {
        if let Some(greeting) = di.get::<String>() {
            println!("Update system says: {}", greeting);
        }
        let mut q = world.query_mut::<Position>();
        for (_ent, pos) in q.iter() {
            pos.0 += 1.0;
            pos.1 += 1.0;
            println!("Updated Position to: {:?}", pos);
        }
    });

    app.startup();
    for _ in 0..3 {
        app.update();
    }
}

fn test_time_in_di() {
    let _ = env_logger::try_init();
    let mut di = DiContainer::new();
    di.insert(TimeState::new());

    for _ in 0..5 {
        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(16));

        let time_state = di.get_mut::<TimeState>().unwrap();
        let time: Time = time_state.tick();
        println!(
            "Delta seconds: {:.4}, Elapsed seconds: {:.4}",
            time.delta_seconds(),
            time.elapsed_seconds()
        );
    }
}

fn test_events() {
    let _ = env_logger::try_init();
    let mut di = DiContainer::new();
    di.insert(Events::<Foo>::new());

    //送信
    {
        let ev = di.get_mut::<Events<Foo>>().unwrap();
        ev.send(Foo(1));
        ev.send(Foo(2));
        println!("drain before update(should be empty):");
        for Foo(n) in ev.drain() {
            println!(" unexpected: {}", n);
        }
    }

    //フレーム末尾:スワップ
    di.get_mut::<Events<Foo>>().unwrap().update();

    //フレームN+1: readバッファから消費
    {
        let ev = di.get_mut::<Events<Foo>>().unwrap();
        println!("drain after update(should have 1,2):");
        for Foo(n) in ev.drain() {
            println!(" received: {}", n);
        }
    }
    //もう一度送信
    {
        let ev = di.get_mut::<Events<Foo>>().unwrap();
        ev.send(Foo(3));
    }
    di.get_mut::<Events<Foo>>().unwrap().update();
    {
        let ev = di.get_mut::<Events<Foo>>().unwrap();
        println!("drain after second update(should have 3):");
        for Foo(n) in ev.drain() {
            println!(" received: {}", n);
        }
    }
}

fn test_ecs() {
    let _ = env_logger::try_init();

    let mut world = ecs::World::new();

    // Spawn entity with Position only, then insert Velocity later
    let e = world.spawn(Position(0.0, 0.0));
    let _ok = world.insert(e, Velocity(1.0, 0.5));

    // Query immutable positions
    {
        let mut q = world.query_ref::<Position>();
        for (ent, pos) in q.iter() {
            println!("[query_ref] entity={:?} pos={:?}", ent, pos);
        }
    }

    // Query mutable velocities and update positions using a second pass
    {
        // Update velocities
        let mut qv = world.query_mut::<Velocity>();
        for (_ent, vel) in qv.iter() {
            // simple scale for demonstration
            vel.0 *= 1.1;
            vel.1 *= 1.1;
        }
    }

    // Simple Euler step: read velocities, then update positions
    // Since our facade currently only supports single-component queries,
    // run in two passes to avoid mutable aliasing complexities.
    let mut velocities: Vec<(ecs::Entity, Velocity)> = {
        let mut qv = world.query_ref::<Velocity>();
        qv.iter().map(|(e, v)| (e, *v)).collect()
    };

    for (e, v) in velocities.drain(..) {
        // Fetch position mutably via query_mut pass
        // Workaround: iterate positions mutably and match the same entity
        let mut qp = world.query_mut::<Position>();
        for (pe, p) in qp.iter() {
            if pe == e {
                p.0 += v.0;
                p.1 += v.1;
            }
        }
    }

    // Inspect results
    {
        let mut q = world.query_ref::<Position>();
        for (ent, pos) in q.iter() {
            println!("[after step] entity={:?} pos={:?}", ent, pos);
        }
    }

    // Remove a component as a final check
    let removed = world.remove::<Velocity>(e);
    println!("removed velocity? {}", removed.is_some());
}
