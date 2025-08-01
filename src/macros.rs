#[macro_export]
macro_rules! simple_fakers {
    ($($module:ident, $faker:ident, $type:ty  $(,$arg:expr)*)*) => {
        use chrono::Duration;
        const SIMPLE_FAKERS: &[&str]=  &[
            $(stringify!($faker),)*
        ];

        pub fn fake(faker: &str) -> Option<String> {

            match (faker) {
                $(stringify!($faker) => Some(
                    fake::faker::$module::fr_fr::$faker(
$($arg,)*
                        ).fake::<$type>().to_string()
                        ), )*
                _ => call_fake(faker),
            }

        }
    };
}

#[macro_export]
macro_rules! call_fakers {
    ($($module_str:ident, $faker_str:ident)*, $($module_vec:ident, $faker_vec:ident)*) => {
        const CALL_FAKERS: &[&str] = &[
            $(stringify!($faker_str),)*
            $(stringify!($faker_vec),)*
        ];

        pub fn call_fake(faker: &str) -> Option<String> {
            match (faker) {
                $(stringify!($faker_str) => Some(fake::faker::$module_str::fr_fr::$faker_str(1..2).fake::<String>()),)*
                $(stringify!($faker_vec) => Some(format!("{:?}", fake::faker::$module_vec::fr_fr::$faker_vec(1..2).fake::<Vec<String>>())),)*
                    _ => None,

            }
        }
    };
}
