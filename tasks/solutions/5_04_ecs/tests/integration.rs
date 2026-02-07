use solution::*;

#[test]
fn create_and_get_component() {
    struct Position { x: f64, y: f64 }
    struct Velocity { dx: f64, dy: f64 }

    let mut world = World::new();
    let e = world.create_entity();
    world.add_component(e, Position { x: 1.0, y: 2.0 });
    world.add_component(e, Velocity { dx: 0.5, dy: -0.5 });

    let pos = world.get_component::<Position>(e).unwrap();
    assert!((pos.x - 1.0).abs() < 1e-9);
    assert!((pos.y - 2.0).abs() < 1e-9);

    let vel = world.get_component::<Velocity>(e).unwrap();
    assert!((vel.dx - 0.5).abs() < 1e-9);
}

#[test]
fn query_single_type() {
    #[derive(Debug, PartialEq)]
    struct Health(i32);

    let mut world = World::new();
    let e1 = world.create_entity();
    let e2 = world.create_entity();
    let e3 = world.create_entity();
    world.add_component(e1, Health(100));
    world.add_component(e3, Health(50));

    let results = world.query::<Health>();
    assert_eq!(results.len(), 2);
    let mut hp_vals: Vec<i32> = results.iter().map(|(_, h)| h.0).collect();
    hp_vals.sort();
    assert_eq!(hp_vals, vec![50, 100]);
}

#[test]
fn replace_component() {
    #[derive(Debug, PartialEq)]
    struct Name(String);

    let mut world = World::new();
    let e = world.create_entity();
    world.add_component(e, Name("Alice".into()));
    world.add_component(e, Name("Bob".into()));

    let name = world.get_component::<Name>(e).unwrap();
    assert_eq!(name.0, "Bob");
}

#[test]
fn remove_component() {
    struct Tag;

    let mut world = World::new();
    let e = world.create_entity();
    world.add_component(e, Tag);
    assert!(world.get_component::<Tag>(e).is_some());
    assert!(world.remove_component::<Tag>(e));
    assert!(world.get_component::<Tag>(e).is_none());
    assert!(!world.remove_component::<Tag>(e));
}

#[test]
fn destroy_entity() {
    struct Marker(u32);

    let mut world = World::new();
    let e = world.create_entity();
    world.add_component(e, Marker(1));
    assert!(world.destroy_entity(e));
    assert!(world.get_component::<Marker>(e).is_none());
    assert!(!world.destroy_entity(e));
    assert_eq!(world.query::<Marker>().len(), 0);
}

#[test]
fn missing_component_returns_none() {
    struct Invisible;
    struct Visible;

    let mut world = World::new();
    let e = world.create_entity();
    world.add_component(e, Visible);
    assert!(world.get_component::<Invisible>(e).is_none());
}
