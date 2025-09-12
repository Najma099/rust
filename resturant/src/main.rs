mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}

        mod child {
            pub fn receive() {
                println!("Okay its working!!!");
            }
        }

        pub fn do_something() {
            child::receive();
        }
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn main() {
    front_of_house::hosting::do_something();
}