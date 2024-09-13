use std::collections::HashMap;

fn group_value_by_key(vec:Vec<(String,i32)>)->HashMap<String,i32>{
    let mut hm = HashMap::new();

    for (key,value) in vec{
        hm.insert(key,value);
    }

    return hm;
}

fn main() {

    let input_vec = vec![(String::from("Deepanshu"),1),(String::from("Deepanshu"),2),(String::from("Deepanshu"),3),(String::from("Deepanshu"),4),(String::from("Deepanshu"),5),(String::from("Deepanshu"),6)];

    let hm = group_value_by_key(input_vec);

    println!("{:?}",hm);

}


//write a function that takes a vector of tuples(each tuple containing a key and value) and returns a hashmap where the keys are the unique keys from the input tuple and the values are the vectors of all corresponding values associated with the key


// insert -> insert a key value pair in the hashmap
// get -> get the value associated with the key
// remove -> remove the key value pair from the hashmap
//clear -> remove all the key value pairs from the hashmap
