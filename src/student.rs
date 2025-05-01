// this module defines the Student struct and provides a constructor for creating student instances with attributes relevant to academic performance and personal background.

// derive debug, partialeq, and clone traits for the Student struct
#[derive(Debug, PartialEq, Clone)]
pub struct Student {
    pub id: String,
    pub total_score: f64,
    pub attendance: f64,
    pub parent_education: f64,
    pub family_income: f64,
    pub stress_level: f64,
}

impl Student {
    // constructor to create a new Student instance with provided values
    pub fn new(id: String, total_score: f64, attendance: f64, parent_education: f64, family_income: f64, stress_level: f64) -> Self {
        Student {
            // initialize fields
            id,
            total_score,
            attendance,
            parent_education,
            family_income,
            stress_level,
        }
    }
}



