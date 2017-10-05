use mysql;

error_chain!{
    foreign_links {
        MYSQL(mysql::Error);
    }
}
