fn main() {
    let ter = dec_to_ternaire(100);
    println!("{}", ter);
    let dec = ter_to_decimal("1T01T");
    println!("{}",dec);
    
}

fn dec_to_ternaire(a: i32) -> &str{
    let mut current_value = a;
    let mut result = String::new();
    if current_value == 0 {
        return "0".to_string();
    }
    while current_value != 0 {
        let rest_current_value = current_value % 3;
            if rest_current_value == 0{
                result = rest_current_value.to_string() + &result;
            }
            else if rest_current_value == 1 {
                result = rest_current_value.to_string() + &result;
            }else if rest_current_value ==2 {
                result = "T".to_owned() + &result;
                current_value += 1;
            }
            current_value /= 3;
    }
    result
}   

fn ter_to_decimal(a:&str)->i32{
    let mut result = 0;
    let length = a.len()-1;
    println!("{}",length);

    for (i,c) in a.char_indices(){
        if c == '1' {
            result = result + (3_i32.pow( length as u32 - i as u32)) as i32;
        }
        else if c == 'T'{
            result = result - (3_i32.pow( length as u32 - i as u32)) as i32;
        }
    }
    result
}

fn add_ter(a:&str,b:&str)->&str




