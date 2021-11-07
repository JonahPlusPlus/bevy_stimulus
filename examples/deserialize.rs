use bevy_stimulus::*;

#[stimulus]
enum MyProfile {
    Hello
}

#[stimulus]
enum MyEvent {
    Stomp,
    Push
}


fn main() {
    let result = toml::from_str::<Stimulus<MyProfile, MyEvent>>("[Hello.Push]\ndigital = \"S\"\n\n[Hello.Stomp]\ndigital = \"LShift\"\nscale = 2.0");

    match result {
        Ok(s) => {
            let sc = s.get_scheme(&MyProfile::Hello).unwrap();
            let a = sc.get_action(&MyEvent::Stomp).unwrap();
            println!("{:?}", a);
        }
        Err(e) => {
            println!("{}", e);
        }
    }

}