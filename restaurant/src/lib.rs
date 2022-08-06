mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}
    }
}

use crate::front_of_house::serving::serve_order;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_wait_list();
    front_of_house::hosting::add_to_wait_list();
}
