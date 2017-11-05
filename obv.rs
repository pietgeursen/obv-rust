extern crate uuid;
use std::collections::HashMap;
use uuid::Uuid;

struct Obv<T>{
    value: T,
    subscribers: HashMap<Uuid, Box<Fn(&T)>>,
}

impl<T> Obv<T>{
    pub fn set(&mut self, new_value: T){
        self.value = new_value; 
        let subs = &self.subscribers;
        for f in subs.values(){
            f(&self.value);
        };
    }
    pub fn get(&self) -> &T{
        &self.value 
    }
    pub fn subscribe(&mut self, subscriber: Box<Fn(&T)>) -> Uuid{
        let uuid = Uuid::new_v4();
        &self.subscribers.insert(uuid, subscriber);
        uuid
    }
    pub fn unsubscribe(&mut self, uuid: &Uuid){
        &self.subscribers.remove(&uuid);
    }
}

#[cfg(test)]    
mod test {
    use Obv;
    use std::sync::mpsc::{Sender, Receiver};
    use std::sync::mpsc;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn obv_get_returns_value(){
        let obv: Obv<i32> = Obv{value: 3, subscribers: HashMap::new()};
        assert!(*obv.get() == 3);
    }

    #[test]
    fn obv_value_value_is_updateable(){
        let mut obv: Obv<i32> = Obv{value: 3, subscribers: HashMap::new()};
        assert!(obv.value == 3);
        obv.value = 2;
        assert!(obv.value == 2);
    }

    #[test]
    fn set_updates_value(){
        let mut obv: Obv<i32> = Obv{value: 3, subscribers: HashMap::new()};
        assert!(obv.value == 3);
        obv.set(2);
        assert!(obv.value == 2);
    }

    #[test]
    fn set_updates_value_and_calls_subscriber(){
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let mut obv: Obv<i32> = Obv{value: 3, subscribers: HashMap::new()};
        assert!(obv.value == 3);
        obv.subscribe(Box::new(move |val| {
            assert!(*val == 2);
            tx.send(1).unwrap();
        }));
        obv.set(2);
        assert!(rx.recv().unwrap() == 1);
    }

    #[test]
    fn set_updates_value_and_calls_subscribers(){
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let other_tx = tx.clone();

        let mut obv: Obv<i32> = Obv{value: 3, subscribers: HashMap::new()};
        assert!(obv.value == 3);
        obv.subscribe(Box::new(move |val| {
            assert!(*val == 2);
            tx.send(1).unwrap();
        }));
        obv.subscribe(Box::new(move |val| {
            assert!(*val == 2);
            other_tx.send(1).unwrap();
        }));
        obv.set(2);
        assert!(rx.recv().unwrap() == 1);
        assert!(rx.recv().unwrap() == 1);
    }

    #[test]
    fn maps_n_shit(){
        let mut map : HashMap<Uuid, Box<Fn()>> = HashMap::new();
        let id = Uuid::new_v4();
        let f = Box::new(||{assert!(true)});
        map.insert(id, f);

        for a in map.values(){
            a(); 
        }
        
    }
    
    #[test]
    fn subscribe_returns_an_unsubscribe_function(){
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let mut obv: Obv<i32> = Obv{value: 3, subscribers: HashMap::new()};
        assert!(obv.value == 3);
        let id = obv.subscribe(Box::new(move |val| {
            assert!(false);
        }));
        obv.unsubscribe(&id);
        obv.set(2);
    }

}
