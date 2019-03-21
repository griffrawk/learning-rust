use crate::a_super_fn;

// a fn defined in sound
pub mod instrument {
    pub fn clarinet() {
        // Function body code goes here
        super::a_super_fn();        // super needed here
    }

    pub mod brass {
        pub fn trumpet() {
            super::super::a_super_fn();         // super::super needed here
            crate::a_super_fn();                // this also works. also dont need use
            super::super::not_instrument::conductor();
        }
    }
}

pub mod not_instrument {
    pub fn conductor() {
        // code
        println!("Hello");
    }
}

pub fn abstract_sound() {
    // body
    a_super_fn();                               // brought into scope by use stmt
}
