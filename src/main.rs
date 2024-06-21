



struct People{
    name:String,
    age:i32,
    color:String,
}

fn print_name(name:&str,color:&str){
    println!("{:?}{:?}",name,color);


}


// derive macro in rust 

#[derive(Debug,Clone,Copy)]
enum Position{
    Manager,
    Supervisor,
    Worker,

}

#[derive(Debug,Copy,Clone)]
struct Employee{
    position: Position,
    work_hour: i64,

}

fn print_employee(emp:&Employee){
    println!("{:?}",emp);
}



// advancaed match 
enum Discount {

    Percent(i32),
    Flat(i32),
}


struct Ticket{
    event :String,
    price : i32,
}



fn main() {

    let person = vec![
        People{
            name:String::from("Arman"),
            age:33,
            color:"red".to_owned(),

        },

        People{
            name:String::from("ali"),
            age:10,
            color:"white".to_owned(),

        },
        People{
            name:String::from("sajad"),
            age:5,
            color:"black".to_owned(),

        },


    ];

    for p in person{
        if p.age<10{

            print_name(&p.name,&p.color);
            println!("{:?}",p.age);

        }
       
    }

    let me: Employee = Employee{
        position:Position::Worker,
        work_hour:40,


    };
    println!("{:?}",me);
    print_employee(&me);
    print_employee(&me);




      // advanced match 
      let n = 3;
      match n {

        3 => println!("three"),
        other => println!("{:?}",other),
          
      }

      let flat = Discount::Flat(2);
      match flat {
        Discount::Flat(2) => println!("two"),
        Discount::Flat(amount) => println!("{:?}",amount),
        _=> (),


          
      }

      let ticket = Ticket{
        event : "concert".to_owned(),
        price : 43,

      };
      
     match  ticket{
        Ticket {  price:43,event } => println!(" event is {:?}",event),

        Ticket { price,.. } => println!("price is {:?}",price),

         
     }



}
