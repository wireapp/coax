use diesel::result::{self, ConnectionError, TransactionError};

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
        Transaction(e: TransactionError<result::Error>) {
            display("transaction error: {}", e)
            cause(e)
            from()
        }
        InvalidData(msg: &'static str) {
            display("invalid data: {}", msg)
        }
    }
}
