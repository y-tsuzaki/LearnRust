mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function(); // OK!
    outermost::middle_secret_function(); // error[E0603]: function `middle_secret_function` is private
    outermost::inside::inner_function(); // error[E0603]: module `inside` is private
    outermost::inside::secret_function(); //error[E0603]: module `inside` is private
}