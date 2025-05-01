// this module provides a function to read student data from a CSV file and return it as a list of Student structs along with the CSV headers, handling errors gracefully during parsing

// import the Student struct, which is used to represent a student
use crate::student::Student;
// import the Error trait to handle errors
use std::error::Error;
// import the csv crate for reading csv files 
use csv;

// this function loads student data from the provided CSV file and returns a tuple containing
// a vector of Student structs
// a vector of column headers from the CSV file
// it returns a Result to handle potential errors during file reading or data parsing
pub fn load_students_from_csv(file_path: &str) -> Result<(Vec<Student>, Vec<String>), Box<dyn Error>> {
    // create a CSV reader for the provided file path
    let mut rdr = csv::Reader::from_path(file_path)?;
    // initialize empty vectors to hold student data and CSV headers
    let mut students = Vec::new();
    let mut headers = Vec::new();

    // attempt to read the CSV headers
    // if successful, store the headers as strings in the "headers" vector
    match rdr.headers() {
        Ok(header) => {
            // convert headers from &str to String and collect into the header vector
            headers = header.iter().map(|s| s.to_string()).collect();
        },
        Err(err) => {
            // if reading the header fails, return the error as a Boxed Error
            return Err(Box::new(err));
        }
    }

    // loop through each record (row) in the CSV file
    for result in rdr.records() {
        // if reading a record succeeds, parse it into a student
        let record = result?;
        // extract and convert values from the CSV record to their respective types
        // Student_ID (first column)
        let id = record[0].to_string();
        // Total_Score (10th column, indec 9)
        let total_score: f64 = record[9].parse()?;
        // Attendance (3rd column, index 2)
        let attendance: f64 = record[2].parse()?;
        // Parent_Education (17th column, index 16)
        let parent_education: f64 = record[16].parse()?;
        // Family_Income (18th column, index 17)
        let family_income: f64 = record[17].parse()?;
        // Stress_Level (13th column, index 12)
        let stress_level: f64 = record[12].parse()?;

        // create a new Student instance using the parsed data
        let student = Student::new(id, total_score, attendance, parent_education, family_income, stress_level);
        // push the newly created student into the students vector
        students.push(student);
    }

    // return the vector of students along with the headers as a tuple
    Ok((students, headers))
}
