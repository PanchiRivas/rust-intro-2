//each property and it type
struct Data{
    num1: i32,
    num2: i32,
    str1: String,
    optional_num: Option<i32>
}

//specify the type of the tuple
struct TwoNums(i32, i32);

//unit struct
struct Calculator;

//implement methods on a struct
impl Data {
    fn new() -> Self{
        Data { //default
            num1: 2,
            num2: 2,
            str1: "some string".to_string(),
            optional_num: Some(8)
        }
    }
    fn sum(&self) -> i32{
        self.num1 + self.num2
    }
}

//implement methods on a unit struct
impl Calculator{ // the '->' means return
    fn add(n1: i32, n2: i32) -> i32{
        n1+n2
    }
    fn sub(n1: i32, n2: i32) -> i32{
        n1-n2
    }
    fn mul(n1: i32, n2: i32) -> i32{
        n1*n2
    }
    fn div(n1: f32, n2: f32) -> f32{
        (n1/n2) //to return a float '() as f32')
    }
}


fn main() {
    let a = Data{
        num1: 4,
        num2: 3,
        str1: "whatever".to_string(),
        optional_num: None
    };
    println!("{}", a.sum());

    //if i want to use all the default struct but one datum
    let b = Data{ num1: 8, ..a};
    println!("{}", b.sum());

    let mut c = Data::new();
    c.num1 = 3;
    println!("{}", c.sum());

    let d = TwoNums(4, 3);

    //access de numbers in a truple struct
    println!("{}, {}",d.0,d.1);

    println!("{}", Calculator::add(3, 2));
    println!("{}", Calculator::sub(3, 2));
    println!("{}", Calculator::mul(3, 2));
    println!("{}", Calculator::div(3.0, 2.0));

}
