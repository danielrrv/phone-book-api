// #[cfg(test)]
// use crate::collection;
mod tests {
    use macros_utils::collection;
    use syn::parse_macro_input;
    #[test]
    fn first_test() {
        assert_eq!(1, 1)
    }
    #[test]
    fn collection_test() {
        
        #[collection("businesses")]
        struct Home {
            owner: String
        }
        // let home = Home{mock: String::from("Cosas de la vida"),owner: String::from("Daniel")};
        // assert_eq!(String::from("Cosas de la vida"), home.mock)
    }
}
