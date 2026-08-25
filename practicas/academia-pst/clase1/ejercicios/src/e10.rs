fn animal_habitat(animal: &str) -> &str {

    let mut identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2.0
    } else if animal == "snake" {
        3
    } else {
        "Unknown"
    };

    // No cambies la siguiente expresión
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}


//////////////////////////////////////
//
// NO MODIFIQUES LOS SIGUIENTES TESTS
//
#[test]
fn gopher_lives_in_burrow() {
    assert_eq!(animal_habitat("gopher"), "Burrow")
}

#[test]
fn snake_lives_in_desert() {
    assert_eq!(animal_habitat("snake"), "Desert")
}

#[test]
fn crab_lives_on_beach() {
    assert_eq!(animal_habitat("crab"), "Beach")
}

#[test]
fn unknown_animal() {
    assert_eq!(animal_habitat("dinosaur"), "Unknown")
}