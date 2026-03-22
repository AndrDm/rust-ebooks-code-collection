use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;

struct Log;

impl Log {
    pub fn info(&self, s: String) {
        use std::io::{stdout, Write};

        let _ = stdout().write_fmt(format_args!("[LOG] {s}"));
    }
}

pub struct ServiceItem(Rc<dyn Any>, TypeId);

impl ServiceItem {
    pub fn new<T: Any>(service: T) -> Self {
        // TypeId (&self) anfordern,
        // bevor der Move an RefCell stattfindet
        let type_id = service.type_id();
        Self {
            0: Rc::new(service),
            1: type_id,
        }
    }
}

struct ServiceContainer {
    services: HashMap<TypeId, ServiceItem>,
}

impl ServiceContainer {
    pub fn new() -> ServiceContainer {
        ServiceContainer {
            services: HashMap::new(),
        }
    }

    pub fn registriere(&mut self, s: ServiceItem) {
        self.services.insert(s.1, s);
    }

    pub fn finde<T: 'static>(&self) -> Option<Rc<T>> {
        let opt_service = self.services.get(&TypeId::of::<T>());

        if let Some(service) = opt_service {
            let service_item = service.0.clone();
            let option = service_item.downcast::<T>();
            option.ok()
        } else {
            None
        }
    }
}

pub fn main() {
    let mut container = ServiceContainer::new();

    let log = ServiceItem::new(Log);
    container.registriere(log);

    let opt_service_log: Option<Rc<Log>> = container.finde();
    opt_service_log.unwrap().info("Hallo, Rust".into());

    println!();

    // Veränderliches Trait-Objekt von veränderlicher i32 Referenz
    let mut zahl: &mut dyn Any = &mut 32;
    let ist_zahl: bool = zahl.is::<i32>();
    assert!(ist_zahl);

    let ist_string: bool = zahl.is::<String>(); // false
    assert!(!ist_string);

    let zahl_ref: &i32 = zahl.downcast_ref::<i32>().unwrap();
    assert_eq!(*zahl_ref, 32);

    let zahl_mut: &mut i32 = zahl.downcast_mut::<i32>().unwrap();
    *zahl_mut = 42;
    assert_eq!(*zahl_mut, 42);
}
