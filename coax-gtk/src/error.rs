use coax_actor;
use coax_data;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidAppState {
            display("invalid application state")
        }
        Actor(e: coax_actor::Error) {
            display("actor error: {}", e)
            cause(e)
            from()
        }
        Database(e: coax_data::Error) {
            display("database error: {}", e)
            cause(e)
            from()
        }
    }
}

