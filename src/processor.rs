pub mod data {
    use crate::client;
    use serde::{Serialize, Deserialize};
    use chrono::NaiveDate;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Data {
        courses: Vec<Course>,
    }

    impl Data {
        pub fn new() -> Self {
            Data {
                courses: Vec::new(),
            }
        }
        
        pub fn insert_course(&mut self, course: Course) {
            self.courses.push(course);
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Course {
        name: String,
        categories: Vec<Category>,
        assignments: Vec<Assignment>,
    }

    impl Course {
        pub fn new(name: String) -> Self {
            Course {
                name,
                categories: Vec::new(),
                assignments: Vec::new(),
            }
        }

        pub fn insert_assign(mut self, assignment: Assignment) -> Self {
            self.assignments.push(assignment);
            self
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Category {
        pub cat_type: String,
        weight: f32,
        points: f32,
        possible: f32,
    }

    impl Category {
        pub fn new(cat_type: String, weight: f32, points: f32, possible: f32) -> Self {
            Category {
                cat_type,
                weight,
                points,
                possible,
            }
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Assignment {
        date: NaiveDate,
        assign: String,
        category: String,
        score: f32,
        score_possible: f32,
        score_type: String,
        points: f32,
        points_possible: f32,
    }

    impl Assignment {
        pub fn new(date: NaiveDate, assign: String, category: String, score: f32, score_possible: f32, 
        score_type: String, points: f32, points_possible: f32) -> Self {
            Assignment {
                date,
                assign,
                category,
                score,
                score_possible,
                score_type,
                points,
                points_possible,
            }
        }
    }

    pub fn process(data: Data) -> Data {

        data
    }
}
