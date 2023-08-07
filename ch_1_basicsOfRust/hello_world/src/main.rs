#[allow(unused_variables)] //attributes to suppress compiler warning for specific scenarios
#[allow(unused_assignments)]
#[allow(dead_code)]


    
fn max(a:i32, b:i32) ->i32{ //functions are declared using keyword fn
                            //-> is used to specify a return type
    if a>b{
        a
    }
    else {
        b
    }
} 
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]


fn main(){  //main function
    let number=10; //variables are immutable by default
    let mut mutable_number=20; //use mut keyword to make mutable
    mutable_number+=1;         //works without any errors
    println!("{}",max(40,444));//println! is a macro which is different from a function
                              //{} specifies content, which here is max 

    let array: [i32;4]=[1,2,3,4]; //a fixed size array
    let mut vector: Vec<i32>=vec![1,2,3,4]; //dynamic array (vector)
    vector.push(5);


    for i in array{             //for loop, iterates over above array and prints every element
        println!("{}", i);
    }


}


