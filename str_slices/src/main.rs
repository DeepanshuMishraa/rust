

fn main(){

    let mut  word = String::from("hello world");

    let word2 = &word[0..5]; // hello. it will print the first 5 characters of the string

    word.clear(); // it will throw an error as we are trying to modify the string after taking a slice of it

    // as word2  will be holding a reference to the word and if we try to modify the word then it will throw an error as the reference is still in use

    //so while you have an immutable reference to a value, you can't have a mutable reference to the same value

    // basically word2 will become a dangling reference as the value it was pointing to has been deallocated


    println!("{}",word2);

}


fn find_first_word(word:String) -> &str{
    let index = 0;

    for (_,i) in word.chars().enumerate(){
        if i == ''{
            break;
        }
        index+=1;
    }

    return &word[0..index];
}
