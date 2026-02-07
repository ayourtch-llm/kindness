use std::any::{Any, TypeId};

pub trait Event: Any + 'static {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriptionId(pub u64);

struct Subscription {
    id: SubscriptionId,
    type_id: TypeId,
    handler: Box<dyn Fn(&dyn Any)>,
}

pub struct EventBus {
    subscriptions: Vec<Subscription>,
    next_id: u64,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            subscriptions: Vec::new(),
            next_id: 0,
        }
    }

    pub fn subscribe<E: Event>(&mut self, handler: Box<dyn Fn(&E)>) -> SubscriptionId {
        let id = SubscriptionId(self.next_id);
        self.next_id += 1;
        let type_id = TypeId::of::<E>();
        let wrapper: Box<dyn Fn(&dyn Any)> = Box::new(move |any: &dyn Any| {
            if let Some(event) = any.downcast_ref::<E>() {
                handler(event);
            }
        });
        self.subscriptions.push(Subscription {
            id,
            type_id,
            handler: wrapper,
        });
        id
    }

    pub fn publish<E: Event>(&self, event: &E) {
        let type_id = TypeId::of::<E>();
        for sub in &self.subscriptions {
            if sub.type_id == type_id {
                (sub.handler)(event as &dyn Any);
            }
        }
    }

    pub fn unsubscribe(&mut self, id: SubscriptionId) {
        self.subscriptions.retain(|sub| sub.id != id);
    }
}
