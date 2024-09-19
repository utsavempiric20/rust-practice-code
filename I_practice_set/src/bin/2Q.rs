enum JobStaus {
    Employed,
    Unemployed,
    Retired,
}
struct Person {
    name: String,
    age: i32,
    job: JobStaus,
}

impl Person {
    fn check_jobstatus(&self) {
        match self.job {
            JobStaus::Employed => println!("{} is {} year old Employed", self.name, self.age),
            JobStaus::Unemployed => println!("{} is {} year old Unemployed", self.name, self.age),
            JobStaus::Retired => println!("{} is {} year old Retired", self.name, self.age),
        }
    }
}

fn main() {
    let person1 = Person {
        name: String::from("ABC"),
        age: 20,
        job: JobStaus::Employed,
    };
    let person2 = Person {
        name: String::from("DEF"),
        age: 22,
        job: JobStaus::Unemployed,
    };
    let person3 = Person {
        name: String::from("XYZ"),
        age: 85,
        job: JobStaus::Retired,
    };
    person1.check_jobstatus();
    person2.check_jobstatus();
    person3.check_jobstatus();
}
