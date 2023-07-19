use std::collections::{HashMap, HashSet};

// 班级
#[derive(Debug)]
pub struct Class {
    pub id: u32,
    name: String,
    student_ids: HashSet<u32>,
}

impl Class {
    pub fn new(id: u32, name: String) -> Self {
        Class {
            id,
            name,
            student_ids: Default::default(),
        }
    }
}

impl Clone for Class {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            student_ids: self.student_ids.clone(),
        }
    }
}

#[derive(Debug)]
pub struct DbClass {
    pub classes: HashMap<u32, Class>,
}

impl DbClass {
    // 添加班级
    pub fn add_class(&mut self, class: Class) {
        self.classes.insert(class.id, class);
    }

    // 删除班级
    fn del_class(&mut self, id: u32) {
        self.classes.remove(&id);
    }

    // 更新班级
    fn update_class(&mut self, class: Class) {
        self.classes.insert(class.id, class);
    }

    // 获取班级
    pub fn get_class(&self, id: u32) -> Option<&Class> {
        self.classes.get(&id)
    }

    // 班级添加学生
    pub fn add_student(&mut self, class_id: u32, student_id: u32) {
        self.classes
            .get_mut(&class_id)
            .unwrap()
            .student_ids
            .insert(student_id);
    }
}
