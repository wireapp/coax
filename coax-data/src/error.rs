use diesel::migrations::RunMigrationsError;
use diesel::result::{self, ConnectionError};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Conn(e: ConnectionError) {
            display("connection error: {}", e)
            cause(e)
            from()
        }
        Result(e: result::Error) {
            display("result error: {}", e)
            cause(e)
            from()
        }
        InvalidData(msg: &'static str) {
            display("invalid data: {}", msg)
        }
        Migration(e: RunMigrationsError) {
            display("migration error: {}", e)
            cause(e)
            from()
        }
        InvalidPath
    }
}
