#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[cfg(feature = "feat-c")]
    compile_error!("Default feat-c NOT disabled");
}
