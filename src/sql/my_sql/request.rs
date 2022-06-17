use crate::sql::schema::{CHEAPTable, Challenge, Session, User};
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
            `pubkey` varchar(1000) COLLATE 'ascii_bin' NOT NULL
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

    pub fn prepare_retrieve_id() -> String {
        format!(r#"DELIMITER //

        DROP PROCEDURE IF EXISTS retrieve;
        CREATE PROCEDURE retrieve(arg VARCHAR(20))
        BEGIN
           IF (SELECT COUNT(*) FROM `{}` WHERE `name`=arg) > 0
           THEN
              SELECT `id` FROM `{}` WHERE `name`=arg;
           ELSE
              SELECT COUNT(`id`) FROM `{}` WHERE `name`=arg;
           END IF;
        END 
        //
        DELIMITER ;"#,&Self::table(),&Self::table(),&Self::table())
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

/* ########################  Session  ########################### */

impl Session {
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
        CHEAPTable::Sessions.name()
    }
}

pub fn demo() {
    let tr = CHEAPTransaction::new(SQL_ROOT.to_string());

    let reqs = vec![
        User::prepare_up_table(),
        Session::prepare_up_table(),
        Challenge::prepare_up_table(),
    ];
    // () means "no params"
    let params = vec![(), (), ()];
    if !tr.full_exec(reqs, params) {
        // You can just create a pool without instantiate a full CHEAPTransaction
        //let pool = CHEAPTransaction::pool(SQL_ROOT.to_string());

        // by calling succesive single_exec, you'll modify tr -> don't forget the mut
        let mut tr = CHEAPTransaction::new(SQL_ROOT.to_string());

        tr.single_exec(Session::prepare_down_table(), ());
        tr.single_exec(Challenge::prepare_down_table(), ());
        tr.last_exec(User::prepare_down_table(), ());
        // or a third last_exec then :
        //tr.transaction.commit();
    }
}

/*
pub fn dev_insert() { 
    let user = User{id: 0, name: String::from("lama"), pubkey: fs::read_to_string(std::path::Path::new("data/dev.pub")).expect("dev.pub not found") }
    let user_req = format!(
        r#"CREATE TABLE `{}` (
        `user_id` int unsigned NOT NULL,
        `nonce` int unsigned NULL,
        `expire` bigint unsigned NOT NULL,
        FOREIGN KEY (`user_id`) REFERENCES `Users` (`id`) ON DELETE CASCADE
      )"#,
        &Self::table()
    )
}*/