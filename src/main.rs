//Structs
struct User{
    username: String,
    email: String,
    sign_in_cnt: u32,
    active: bool
}

//New types
struct Coord(i16, i16, i16);

fn main() {
    // types don't matter whether you use & or not as it's stack data
    let tup = (1, 2, 3);
    let (a, b, c) = tup;
    println!("{} {} {}", &a, &b, &c);

    let usr_arnob = User{
        username: String::from("arnob"),
        email: String::from("a@a.com"),
        sign_in_cnt: 1,
        active: false
    };

    println!("{} {} {} {}", usr_arnob.username, usr_arnob.email, usr_arnob.sign_in_cnt, usr_arnob.active);

    let usr_bob = User{
        username: String::from("bob"),
        email: String::from("ab@a.com"),
        ..usr_arnob
    };    
    
    println!("{} {} {} {}", usr_bob.username, usr_bob.email, usr_bob.sign_in_cnt, usr_bob.active);

    //usr_arnob.username = String::from("arn1");

    let usr_name = String::from("new");
    let email = String::from("b@b.com");

    let new_user = build_usr(usr_name, email);

    println!("{} {} {} {}", new_user.username, new_user.email, new_user.sign_in_cnt, new_user.active);

    let new_coord = Coord(1, 2, 3);

    println!("{} {} {}", new_coord.0, new_coord.1, new_coord.2);
}


fn build_usr(usr_name: String, email: String)->User {
    let new_usr = User{
        username: usr_name,
        email: email,
        sign_in_cnt: 0,
        active: false,
    };

    new_usr
}
