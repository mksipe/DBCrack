//   Copyright (C) 2020  Mason Sipe <m-sipe@protonmail.com>
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::path::Path;

pub fn db_exists() {
    if Path::new("db.sqlite3").exists() == true {
        println!("{}", "[ ENABLED  ] Database Ready")
    } else {
        println!("{}", "[ DISABLED ] Database Ready")
    }
}
pub fn db_scheme_exists() {
    if Path::new("db.yaml").exists() == true {
        println!("{}", "[ ENABLED  ] Database Scheme Present")
    } else {
        println!("{}", "[ DISABLED ] Database Scheme Present")
    }
}
