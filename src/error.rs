use mysql;
use std;

error_chain!{
    foreign_links {
        MySQLError(mysql::Error);
        IOError(std::io::Error);
    }
}
