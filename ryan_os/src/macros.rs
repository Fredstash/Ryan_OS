#[macro_export]
macro_rules! add {
 // the parentheses of a macro defined with $a and $b
    ($a:expr,$b:expr) => {
        {
            // takes the arguments from above and returns the expression
            $a + $b
        }
    };
    // if the user only adds on input just display the input
    ($a:expr) => {
        $a
    }
}

#[macro_export]
macro_rules! sub {
 // the parentheses of a macro defined with $a and $b
    ($a:expr,$b:expr) => {
        {
            // takes the arguments from above and returns the expression
            $a - $b
        }
    };

    // if the user only adds one argument
    ($a:expr) => {
        $a
    }
}