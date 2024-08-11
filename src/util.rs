pub fn zeros (zero_num : u32) -> String {

    let mut output = "".to_string();
    for _ in 0..zero_num{
        output = format!("{}0",output)
    }

    output
}

pub fn num_to_seqstr (num : u32, digit : u32) -> String{
    let zeros = {
        if num < 10 && digit > 0{zeros(digit - 1)}
        else if num < 100 && digit > 1{zeros(digit - 2)}
        else if num < 1000 && digit > 2{zeros(digit - 3)}
        else if num < 10000 && digit > 3{zeros(digit - 4)}
        else if digit > 4 {zeros(digit - 5)}
        else {panic!("invalid digit")}
    };
    
    format!("{}{}",zeros,num)
}