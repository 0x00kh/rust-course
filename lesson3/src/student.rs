use std::collections::HashMap;

//学生
#[derive(Debug, Clone)]
pub struct Student {
    pub id: u32,
    name: String,
}

impl Student {
    pub fn new(id: u32, name: String) -> Self {
        Student { id, name }
    }
}

// impl Clone for Student {
//     fn clone(&self) -> Self {
//         Self {
//             id: self.id,
//             name: self.name.clone(),
//         }
//     }
// }

#[derive(Debug)]
pub struct DbStudent {
    pub students: HashMap<u32, Student>,
}

impl DbStudent {
    // 添加学生
    pub fn add_student(&mut self, student: Student) {
        self.students.insert(student.id, student);
    }

    // 删除学生
    fn del_student(&mut self, id: u32) {
        self.students.remove(&id);
    }

    // 更新学生
    fn update_student(&mut self, student: Student) {
        self.students.insert(student.id, student);
    }

    // 获取学生
    pub fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }
}
