use solution::*;

#[test]
fn subscribe_and_publish() {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::any::Any;

    #[derive(Debug)]
    struct Click { x: i32, y: i32 }
    impl Event for Click {
        fn as_any(&self) -> &dyn Any { self }
    }

    let mut bus = EventBus::new();
    let log: Rc<RefCell<Vec<(i32,i32)>>> = Rc::new(RefCell::new(Vec::new()));
    let log_clone = log.clone();

    bus.subscribe::<Click>(Box::new(move |e: &Click| {
        log_clone.borrow_mut().push((e.x, e.y));
    }));

    bus.publish(&Click { x: 10, y: 20 });
    bus.publish(&Click { x: 30, y: 40 });

    assert_eq!(*log.borrow(), vec![(10,20), (30,40)]);
}

#[test]
fn multiple_event_types() {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::any::Any;

    #[derive(Debug)]
    struct EventA(i32);
    impl Event for EventA {
        fn as_any(&self) -> &dyn Any { self }
    }

    #[derive(Debug)]
    struct EventB(String);
    impl Event for EventB {
        fn as_any(&self) -> &dyn Any { self }
    }

    let mut bus = EventBus::new();
    let a_log: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    let b_log: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let a_clone = a_log.clone();
    let b_clone = b_log.clone();

    bus.subscribe::<EventA>(Box::new(move |e: &EventA| {
        a_clone.borrow_mut().push(e.0);
    }));
    bus.subscribe::<EventB>(Box::new(move |e: &EventB| {
        b_clone.borrow_mut().push(e.0.clone());
    }));

    bus.publish(&EventA(1));
    bus.publish(&EventB("hello".into()));
    bus.publish(&EventA(2));

    assert_eq!(*a_log.borrow(), vec![1, 2]);
    assert_eq!(*b_log.borrow(), vec!["hello".to_string()]);
}

#[test]
fn unsubscribe() {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::any::Any;

    #[derive(Debug)]
    struct Msg(i32);
    impl Event for Msg {
        fn as_any(&self) -> &dyn Any { self }
    }

    let mut bus = EventBus::new();
    let log: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    let log_clone = log.clone();

    let id = bus.subscribe::<Msg>(Box::new(move |e: &Msg| {
        log_clone.borrow_mut().push(e.0);
    }));

    bus.publish(&Msg(1));
    bus.unsubscribe(id);
    bus.publish(&Msg(2));

    assert_eq!(*log.borrow(), vec![1]);
}

#[test]
fn multiple_handlers_same_event() {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::any::Any;

    #[derive(Debug)]
    struct Ping;
    impl Event for Ping {
        fn as_any(&self) -> &dyn Any { self }
    }

    let mut bus = EventBus::new();
    let count: Rc<RefCell<i32>> = Rc::new(RefCell::new(0));
    let c1 = count.clone();
    let c2 = count.clone();

    bus.subscribe::<Ping>(Box::new(move |_: &Ping| {
        *c1.borrow_mut() += 1;
    }));
    bus.subscribe::<Ping>(Box::new(move |_: &Ping| {
        *c2.borrow_mut() += 10;
    }));

    bus.publish(&Ping);
    assert_eq!(*count.borrow(), 11);
}

#[test]
fn unique_subscription_ids() {
    use std::any::Any;

    #[derive(Debug)]
    struct Evt;
    impl Event for Evt {
        fn as_any(&self) -> &dyn Any { self }
    }

    let mut bus = EventBus::new();
    let id1 = bus.subscribe::<Evt>(Box::new(|_: &Evt| {}));
    let id2 = bus.subscribe::<Evt>(Box::new(|_: &Evt| {}));
    let id3 = bus.subscribe::<Evt>(Box::new(|_: &Evt| {}));

    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    assert_ne!(id1, id3);
}

#[test]
fn publish_with_no_subscribers() {
    use std::any::Any;

    #[derive(Debug)]
    struct Ghost;
    impl Event for Ghost {
        fn as_any(&self) -> &dyn Any { self }
    }

    let bus = EventBus::new();
    // Should not panic
    bus.publish(&Ghost);
}
