# rust-new-project-template
A good starting point for a new Rust project
The Rust program utilizes several common Rust features, such as structs, methods, vectors, iterators, and conditional statements. The program creates a simple student management system that allows users to add student information, list all student information, and search for specific student information based on a given ID.

The program defines a Student struct that contains fields for a student's ID, name, age, and score. The Student struct also has an impl block that defines two methods: new() and display(). The new() method is a constructor that creates a new instance of the Student struct, while the display() method prints out the student's information to the console.

The program then creates a vector called students that can hold multiple instances of the Student struct. It initializes the students vector with three Student instances, and then uses a for loop with an iterator to print out all the student information to the console.

Finally, the program searches for a specific student based on a given ID. It uses another for loop with an iterator to search for the student with the given ID, and then prints out the student's information to the console if found. If the student is not found, the program prints out a message indicating that the student with the given ID was not found.
