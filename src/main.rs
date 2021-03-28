fn main() {
    song();
}

fn song(){
    let mut index: i32 = 0;
    let mut aux;

    let phrase_array = ["A partridge in a pear tree", "Two turtle doves","Three French hens","Four calling birds","Five golden rings","Six geese a-laying","Seven swans a-swimming","Eight maids a-milking","Nine ladies dancing","Ten lords a-leaping","Eleven pipers piping","Twelve drummers drumming",""];
   
    while index<12 {
        
        println!("| on the {} day of Christmas, My true love sent to me",index+1);
        aux=index;
        loop {
            if aux>=0 {
                println!("|__*{}",phrase_array[aux as usize]);
                aux-=1;
            }else{
                break;
            }
        }
        println!("------");

        index+=1;

    }
}
