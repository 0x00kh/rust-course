use std::collections::{HashMap, HashSet};

//课程
#[derive(Debug)]
pub struct Course {
    id: u32,
    name: String,
    code: String,
    class_id: u32,
}

impl Course {
    pub fn new(id: u32, name: String, code: String, class_id: u32) -> Self {
        Course{id, name, code, class_id}
    }
}

#[derive(Debug)]
pub struct DbCourse {
    pub courses: HashMap<u32, Course>,
}


impl DbCourse {
    // 添加课程
    pub fn add_course(&mut self, course: Course) {
        self.courses.insert(course.id, course);
    }

    // 删除课程
    fn del_course(&mut self, id:u32) {
        self.courses.remove(&id);
    }

    // 更新课程
    fn update_course(&mut self, course: Course) {
        self.courses.insert(course.id, course);
    }

    // 获取课程
    pub fn get_course(&self, id: u32) -> Option<&Course> {
        self.courses.get(&id)
    }

}