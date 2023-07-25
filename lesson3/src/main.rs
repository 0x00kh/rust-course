use crate::class::{Class, DbClass};
use crate::club::{Club, DbClub};
use crate::course::{Course, DbCourse};
use crate::student::{DbStudent, Student};
use std::collections::HashMap;

mod class;
mod club;
mod course;
mod student;

#[warn(unused_variables)]
fn main() {
    let mut db_student = DbStudent {
        students: HashMap::new(),
    };

    let mut db_class = DbClass {
        classes: HashMap::new(),
    };

    let mut db_course = DbCourse {
        courses: HashMap::new(),
    };

    let mut db_club = DbClub {
        clubs: HashMap::new(),
    };

    let tom = Student::new(1, "Tome".to_string());
    db_student.add_student(tom.clone());

    let class101 = Class::new(1, "101".to_string());
    db_class.add_class(class101.clone());
    db_class.add_student(class101.id, tom.id);

    let math00001 = Course::new(1, "math00001".to_string(), "00001".to_string(), class101.id);
    db_course.add_course(math00001);

    let club00001 = Club::new(1, "club00001".to_string());
    db_club.add_club(club00001.clone());
    db_club.add_student(club00001.id, tom.id);

    // 输出学生、班级、课程、社团信息
    println!("{:?}", db_student.get_student(1));
    println!("{:?}", db_class.get_class(1));
    println!("{:?}", db_course.get_course(1));
    println!("{:?}", db_club.get_club(1));
}
