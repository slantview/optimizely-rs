use std::{any::Any, collections::HashMap, sync::{Mutex, Arc}};

use anyhow::Error;

// Manager is a generic interface for managing notifications of a particular type
pub trait Manager where Self: Sized {
	fn add(callback: fn(Box<dyn Any>)) -> Result<i64, Error>;
	fn remove(id: i64);
	fn send(message: Box<dyn Any>);
}

// AtomicManager adds handlers atomically
#[derive(Clone, Debug, Default)]
struct AtomicManager {
	handlers: Arc<Mutex<HashMap<u32, fn(Box<dyn Any>)>>>,
	counter: Arc<Mutex<u32>>
}

impl AtomicManager {
    // new creates a new instance of the atomic manager
    pub fn new() -> Self {
        return Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
            counter: Arc::new(Mutex::new(0))
        }
    }

    // Add adds the given handler
    pub fn add(&self, new_handler: fn(Box<dyn Any>)) -> Result<i64, Error> {
        let counter = *self.counter.lock();
        counter = counter + 1;
        
        // am.handlers[am.counter] = newHandler
        // return int(am.counter), nil
        Ok(0)
    }

    // Remove removes handler with the given id
    pub fn remove(id: i64) -> Result<(), Error> {
        // am.lock.Lock()
        // defer am.lock.Unlock()

        // handlerID := uint32(id)
        // if _, ok := am.handlers[handlerID]; ok {
        //     delete(am.handlers, handlerID)
        //     return
        // }
        // am.logger.Debug(fmt.Sprintf("Handler for id:%d not found", id))
        Ok(())
    }

    // Send sends the notification to the registered handlers
    pub fn send(notification: Box<dyn Any>) -> Result<(), Error> {
        // copying handler to avoid race condition
        // handlers := am.copyHandlers()
        // for _, handler := range handlers {
        //     handler(notification)
        // }
        Ok(())
    }

    // Return a copy of the given handlers
    fn copyHandlers() -> Vec<fn(Box<dyn Any>)> {
        // am.lock.RLock()
        // defer am.lock.RUnlock()
        // for _, v := range am.handlers {
        //     handlers = append(handlers, v)
        // }
        // return handlers
        Ok(())
    }
}