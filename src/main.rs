fn main() {}


#[cfg(test)]
mod tests {
    use measurement::si::{Length, Time, Velocity};

    #[test]
    fn division() {
        let length = Length::new(20.0); // 20m
        let time = Time::new(5.0); // 5s

        let velocity = length / time;

        assert_eq!(velocity.value(), 4.0);
    }

    #[test]
    fn multiplication() {
        let velocity = Velocity::new(20.0); // 20m/s
        let time = Time::new(5.0); // 5s

        let length = velocity * time;

        assert_eq!(length.value(), 100.0);
    }
}
