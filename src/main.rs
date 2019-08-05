// CVE-2019-12083

use std::{any::TypeId, error::Error, fmt,io};

#[derive(Debug)]
#[derive(Clone)]
struct F(u64,u64,u64,String,u64);

impl fmt::Display for F {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //self.0.fmt(f)	
	write!(f, "F({}, {},{},{},{})", self.0, self.1,self.2,self.3,self.4)
    }
}

impl Error for F {
	 fn type_id(&self) -> TypeId {
        // um, woops?
        TypeId::of::<Boom>()
    }
}


#[derive(Debug)]
#[derive(Clone)]
struct S(u64,String,u64,u64,u64);

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //self.0.fmt(f)	
	write!(f, "S({}, {},{},{},{})", self.0, self.1,self.2,self.3,self.4)
    }
}

impl Error for S {
	 fn type_id(&self) -> TypeId {
        // um, woops?
        TypeId::of::<F>()
    }

}

// Same layout as `S`, but when we set the usizes to `5`, an incorrectly
// downcasted `Boom` as `S` will mean the `String` in `S` will treat the
// `0x5` as a pointer.
#[derive(Debug)]
#[derive(Clone)]
struct Boom(String, u64, u64,u64,u64);

impl fmt::Display for Boom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "boom({}, {},{},{},{})", self.0, self.1,self.2,self.3,self.4)
    }
}

impl Error for Boom {
    fn type_id(&self) -> TypeId {
        // um, woops?
        TypeId::of::<S>()
    }

}


fn menu(){
	println!("You have a magic box.\n1.create\n2.show\n3.edit\n0x520~0x522.magic\n4.exit");
}



fn read_num() -> u64 {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: u64 = n.trim().parse().expect("invalid input");
    n
}

fn read_str() -> String {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    n.trim().to_string()
}

fn create()-> std::boxed::Box<dyn Error>{
    println!("input your name:");
    let name = read_str();
    println!("input num1:");
    let x=read_num();
    println!("input num2:");
    let y=read_num();
    println!("input num3:");
    let z=read_num();
    println!("input num4:");
    let i=read_num();
    let res=Box::new(Boom(name,x,y,z,i));
    res
}

fn show(B:&Box<dyn Error>){
        println!("{}",B);
}

fn main() {
    //dbg!(TypeId::of::<Boom>());
    //dbg!(TypeId::of::<S>());
    //dbg!(TypeId::of::<F>());
    let mut boom: Box<dyn Error> = Box::new(Boom("aaaaaaaaaaaaaaaaaaaa".to_string(), 0, 0,0,0));
    //dbg!(boom.type_id());
    loop{
        menu();
        let n:u64 = read_num();
        if n == 1 {
            boom=create();
            //dbg!(boom.type_id());
            //println!("create"); 
        }   
        else if n==2 {
            show(&boom);
        }
        else if n == 4 {
             std::process::exit(0); 
        }
        else if n == 0x520{
            boom=Box::new(boom.downcast_mut::<S>().unwrap().clone());
        }
        else if n == 0x521{
            boom=Box::new(boom.downcast_mut::<F>().unwrap().clone());
        }        
        else if n == 0x522{
            boom=Box::new(boom.downcast_mut::<Boom>().unwrap().clone());
        }
         else if n==3 {
            if boom.type_id()==TypeId::of::<S>() {
                println!("input your name:");
                let name = read_str();
                println!("input num1:");
                let x=read_num();
                println!("input num2:");
                let y=read_num();
                println!("input num3:");
                let z=read_num();
                println!("input num4:");
                let i=read_num();
     
                let mut f= boom.downcast_mut::<S>().expect("Boom is an S??");
                f.0=x;
                f.1=name;
                f.2=y;
                f.3=z;
                f.4=i;
            }
            if boom.type_id()==TypeId::of::<F>() {
                println!("input your name:");
                let name = read_str();
                println!("input num1:");
                let x=read_num();
                println!("input num2:");
                let y=read_num();
                println!("input num3:");
                let z=read_num();
                println!("input num4:");
                let i=read_num();
                //println!("F");
               let mut f= boom.downcast_mut::<F>().expect("Boom is an F??");
                let p:*mut F=f;
                //dbg!(p);
                f.0=x;
                f.1=y;
                f.2=z;
                f.3=name;
                f.4=i;
           
            }
            if boom.type_id()==TypeId::of::<Boom>() {
                println!("input your name:");
                let name = read_str();
                println!("input num1:");
                let x=read_num();
                println!("input num2:");
                let y=read_num();
                println!("input num3:");
                let z=read_num();
                println!("input num4:");
                let i=read_num();
               let mut f= boom.downcast_mut::<Boom>().expect("Boom is an boom...");
                f.0=name;
                f.1=x;
                f.2=y;
                f.3=z;
                f.4=i;
           
            }
        }
    }
}
