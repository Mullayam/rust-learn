fn is_valid_password(password1: &str, password2: &str) -> bool {
    password1 == password2 
}
fn check_passwd_strenght(password: &str) => i32 {
    let contains_lower_case =false
    0

}

#[cfg(test)]
mod tests {
    use crate::is_valid_password;
    fn test_both_are_same_paassword() {
        assert_eq!(true, is_valid_password("123456", "123456"));
    }
    fn test_both_are_not_same_paassword() {
        assert_eq!(false, is_valid_password("123456", "123"));
    }
  
}
mod passwd_strength {
    use crate::check_passwd_strenght;
    #[test]
    fn passwd_is_weak()   {
        assert_eq!(0, check_passwd_strenght("123456"));
    }
    #[test]
    fn passwd_include_only_a_z_letters() {
        assert_eq!(1, check_passwd_strenght("abcdef"));        
    }
    #[test]
    fn passwd_include_only_cap_a_z_letters() {
        assert_eq!(2, check_passwd_strenght("ABCDEF"));        
    }
    #[test]
    fn passwd_include_only_num() {
        assert_eq!(3, check_passwd_strenght("123456"));        
    }
    #[test]
    fn  passwd_include_only_special() {
        assert_eq!(4, check_passwd_strenght("!@#$"));
    }
    #[test]
    fn passwd_less_than_6_chars() {
        assert_eq!(5, check_passwd_strenght("123"));
    }
    #[test]
    fn passwd_more_than_12_chars() {
        assert_eq!(6, check_passwd_strenght("123456888888441"));
        
    }
}


fn main() {
    println!("Hello, world!");
}
