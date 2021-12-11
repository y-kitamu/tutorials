#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            super::serving::take_order();
            super::super::eat_at_restaurant();
        }
        pub fn seat_at_table() {}
    }

    mod serving {
        pub fn take_order() {}
        fn server_order () {}
        fn take_payment() {}
    }
}


pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::seat_at_table();
}
