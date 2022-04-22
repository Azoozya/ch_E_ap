//https://docs.rs/mysql/17.0.0/mysql/struct.Transaction.html
use crate::sql::backend::{Opts, Params, Pool, Transaction};

pub struct CHEAPTransaction<'a> {
    transaction: Transaction<'a>,
    error: bool,
}

impl<'a> CHEAPTransaction<'a> {
    /* ############################## Init ############################### */
    // https://docs.rs/mysql/17.0.0/mysql/struct.Pool.html#method.new
    pub fn pool<T: Into<Opts>>(database_url: T) -> Pool {
        Pool::new(database_url).expect("Connection to database failed :(")
    }

    pub fn new<T: Into<Opts>>(database_url: T) -> CHEAPTransaction<'a> {
        let tr = CHEAPTransaction::pool(database_url)
            .start_transaction(false, None, None)
            .unwrap();
        CHEAPTransaction {
            transaction: tr,
            error: false,
        }
    }

    /* ############################## Q up ############################### */
    pub fn single_exec<A: AsRef<str>, T: Into<Params>>(&mut self, query: A, params: T) {
        // If an error occur, skip
        // if you are chaining calls to prep_exec, when the first error occur it will skip every next calls
        if self.error {
            return;
        }

        self.error = match self.transaction.prep_exec(query, params) {
            Err(e) => {
                if cfg!(debug_assertions) {
                    println!("{:#?}", e);
                }
                true
            }
            Ok(_) => false,
        };
    }

    pub fn last_exec<A: AsRef<str>, T: Into<Params>>(mut self, query: A, params: T) -> bool {
        self.prep_exec(query, params);
        let ret = !self.error;
        match ret {
            true => self.rollback(),
            false => self.commit(),
        };
        ret
    }

    pub fn full_exec<A: AsRef<str>, T: Into<Params> + std::clone::Clone>(
        mut self,
        query: Vec<A>,
        params: Vec<T>,
    ) -> bool {
        let mut ret = false;
        if query.len() == params.len() {
            for i in 0..query.len() {
                let stmt = &query[i];
                let prm = &params[i];
                self.prep_exec(stmt, prm);
                // If statement cause an error, we rollback...
                if self.error {
                    self.rollback();
                    return ret;
                }
            }
            //..otherwise, if everything gone well, we commit
            self.commit();
            ret = true;
        }
        ret
    }

    /* ############################## Stop ############################### */
    // called at the end
    pub fn commit(self) {
        self.transaction.commit().expect("Unable to commit :(");
    }

    pub fn rollback(self) {
        self.transaction.rollback().expect("Unable to rollback :(");
    }
}
