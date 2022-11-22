use adder::*;

#[should_panic(expected = "Guess value")]
fn integration_grater_than_100() {
    Guess::new(200);
}
