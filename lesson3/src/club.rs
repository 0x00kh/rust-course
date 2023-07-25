use std::collections::{HashMap, HashSet};

//社团
#[derive(Debug, Clone)]
pub struct Club {
    pub id: u32,
    name: String,
    student_ids: HashSet<u32>,
}

impl Club {
    pub fn new(id: u32, name: String) -> Self {
        Club {
            id,
            name,
            student_ids: Default::default(),
        }
    }
}

// impl Clone for Club {
//     fn clone(&self) -> Self {
//         Self {
//             id: self.id,
//             name: self.name.clone(),
//             student_ids: self.student_ids.clone(),
//         }
//     }
// }

#[derive(Debug)]
pub struct DbClub {
    pub clubs: HashMap<u32, Club>,
}

impl DbClub {
    // 添加社团
    pub fn add_club(&mut self, club: Club) {
        self.clubs.insert(club.id, club);
    }

    // 删除社团
    fn del_club(&mut self, id: u32) {
        self.clubs.remove(&id);
    }

    // 更新社团
    fn update_club(&mut self, club: Club) {
        self.clubs.insert(club.id, club);
    }

    // 获取社团
    pub fn get_club(&self, id: u32) -> Option<&Club> {
        self.clubs.get(&id)
    }

    // 社团添加学生
    pub fn add_student(&mut self, club_id: u32, student_id: u32) {
        self.clubs
            .get_mut(&club_id)
            .unwrap()
            .student_ids
            .insert(student_id);
    }
}
