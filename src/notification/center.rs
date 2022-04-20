use std::{collections::HashMap, any::Any};
use anyhow::Error;

use super::{entities::Type, manager::Manager};

// Center handles all notification listeners. It keeps track of the Manager for each pub struct of notification.
pub trait Center where Self: Sized {
	fn add_handler(handler_type: Type, manager: Box<dyn Manager>) -> Result<i64, Error>;
	fn remove_handler(handler_id: i64, handler_type: Type) -> Result<(), Error>;
	fn send(handler_type: Type, message: dyn Any) -> Result<(), Error>;
}

// DefaultCenter contains all the notification managers
#[derive(Clone, Debug, Default)]
pub struct DefaultCenter {
	manager_map: HashMap<Type, Box<dyn Manager>>
}

impl DefaultCenter {

    // new returns a new notification center
    pub fn new() -> Self {
        // decisionNotificationManager := NewAtomicManager()
        // projectConfigUpdateNotificationManager := NewAtomicManager()
        // processLogEventNotificationManager := NewAtomicManager()
        // trackNotificationManager := NewAtomicManager()
        // managerMap := make(HashMap<Type, Manager)>
        // managerHashMap<Decision,  = decisionNotificationManager>
        // managerHashMap<ProjectConfigUpdate,  = projectConfigUpdateNotificationManager>
        // managerHashMap<LogEvent,  = processLogEventNotificationManager>
        // managerHashMap<Track,  = trackNotificationManager>
        // return &DefaultCenter{
        //     managerMap: managerMap,
        // }
        Self {
            ..Default::default()
        }
    }

    // AddHandler adds a handler for the given notification type
    pub fn add_handler(&mut self, handler_type: Type, manager: Box<dyn Manager>) -> Result<i64, Error> {
        self.manager_map.insert(handler_type, manager)
        // return -1, fmt.Errorf("no notification manager found for pub struct %s", notificationType)
    }

    // RemoveHandler removes a handler for the given id and notification type
    pub fn remove_handler(&mut self, id: i64, manager_type: Type) -> Result<(), Error> {
        match self.manager_map.get(&manager_type) {
            Some(manager) => {
                manager.remove(id)
            }
            None => Err(Error::msg(format!("no notification manager found for pub struct {}", &manager_type))),
        }
    }

    // Send sends the given notification payload to all listeners of type
    pub fn send(&self, manager_type: Type, notification: Box<dyn Any>) -> Result<(), Error> {
        // if manager, ok := c.manager_hash_map
        //     manager.Send(notification)
        //     return nil
        // }

        // return fmt.Errorf("no notification manager found for pub struct %s", notificationType)
        Ok(())
    }
}