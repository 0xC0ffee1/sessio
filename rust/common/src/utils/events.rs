use tokio::sync::broadcast;


pub struct EventBus<T> {
    pub sender: broadcast::Sender<T>,
    pub receiver: broadcast::Receiver<T>
}

impl<T: Clone> Default for EventBus<T> {
    fn default() -> Self {
        let (sender, receiver) = broadcast::channel::<T>(32);

        EventBus {
            sender,
            receiver
        }
    }
}

impl<T> Clone for EventBus<T> {
    fn clone(&self) -> Self {
        EventBus {
            sender: self.sender.clone(),
            receiver: self.sender.subscribe(), // Create a new receiver by subscribing again
        }
    }
}

impl<T> EventBus<T> {

    pub async fn subscribe(&self) -> broadcast::Receiver<T> {
        self.sender.subscribe()
    }
    
    pub async fn new_sender(&self) -> broadcast::Sender<T> {
        self.sender.clone()
    }
}
