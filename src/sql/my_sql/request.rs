use crate::sql::schema::{CHEAPTable, Challenge, Cookie, User};
use crate::sql::transaction::CHEAPTransaction;
use crate::{SQL_AUTH, SQL_AUTH_USER, SQL_REGISTER, SQL_REGISTER_USER, SQL_ROOT};
//use crate::sql::backend::{Pool};

/* ########################  User  ########################### */

impl User {
    pub fn prepare_up_table() -> String {
        // Update this function when updating ~/migrations/up.sql
        format!(
            r#"CREATE TABLE `{}` (
            `id` int unsigned NOT NULL AUTO_INCREMENT PRIMARY KEY,
            `name` varchar(20) COLLATE 'ascii_bin' NOT NULL,
            `pubkey` varchar(64) COLLATE 'ascii_bin' NOT NULL
          )"#,
            &Self::table()
        )
    }

    pub fn prepare_down_table() -> String {
        format!("DROP TABLE `{}`", &Self::table())
    }

    pub fn table() -> String {
        CHEAPTable::Users.name()
    }
}

/* ########################  Challenge  ########################### */

impl Challenge {
    pub fn prepare_up_table() -> String {
        // Update this function when updating ~/migrations/up.sql
        format!(
            r#"CREATE TABLE `{}` (
            `user_id` int unsigned NOT NULL,
            `nonce` int unsigned NULL,
            `expire` bigint unsigned NOT NULL,
            FOREIGN KEY (`user_id`) REFERENCES `Users` (`id`) ON DELETE CASCADE
          )"#,
            &Self::table()
        )
    }

    pub fn prepare_down_table() -> String {
        format!("DROP TABLE `{}`", &Self::table())
    }

    pub fn table() -> String {
        CHEAPTable::Challenges.name()
    }
}

/* ########################  Cookie  ########################### */

impl Cookie {
    pub fn prepare_up_table() -> String {
        // Update this function when updating ~/migrations/up.sql
        format!(
            r#"CREATE TABLE `{}` (
            `user_id` int unsigned NOT NULL,
            `expire` bigint unsigned NOT NULL,
            FOREIGN KEY (`user_id`) REFERENCES `Users` (`id`) ON DELETE CASCADE
          )"#,
            &Self::table()
        )
    }

    pub fn prepare_down_table() -> String {
        format!("DROP TABLE `{}`", &Self::table())
    }

    pub fn table() -> String {
        CHEAPTable::Cookies.name()
    }
}

pub fn demo() {
    let tr = CHEAPTransaction::new(SQL_ROOT.to_string());

    let drops = vec![
        User::prepare_up_table(),
        Cookie::prepare_up_table(),
        Challenge::prepare_up_table(),
    ];
    // () means "no params"
    let params = vec![(), (), ()];
    if tr.exec(drops, params) {
        // You can just create a pool without instantiate a full CHEAPTransaction
        //let pool = CHEAPTransaction::pool(SQL_ROOT.to_string());

        // by calling succesive prep_exec, you'll modify tr -> don't forget the mut
        let mut tr = CHEAPTransaction::new(SQL_ROOT.to_string());

        tr.prep_exec(Cookie::prepare_down_table(), ());
        tr.prep_exec(Challenge::prepare_down_table(), ());
        tr.last_prep_exec(User::prepare_down_table(), ());
        // or a third prep_exec then :
        //tr.transaction.commit();
    }
}
