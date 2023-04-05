struct Student {
    id: i32,
    name: String,
    age: i32,
    score: f32,
}

impl Student {
    fn new(id: i32, name: String, age: i32, score: f32) -> Self {
        Self {
            id,
            name,
            age,
            score,
        }
    }

    fn display(&self) {
        println!("ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Score: {}", self.score);
    }
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    let student1 = Student::new(1, String::from("Alice"), 20, 95.5);
    let student2 = Student::new(2, String::from("Bob"), 21, 85.0);

    students.push(student1);
    students.push(student2);

    let student3 = Student::new(3, String::from("Charlie"), 19, 90.0);

    students.push(student3);

    println!("All students:");
    for student in &students {
        student.display();
    }

    let search_id = 2;
    let mut found = false;
    for student in &students {
        if student.id == search_id {
            println!("Student found:");
            student.display();
            found = true;
            break;
        }
    }
    if !found {
        println!("Student with ID {} not found.", search_id);
    }
}
