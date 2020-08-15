//have copy trait to be treated as a type
struct User{
    username: String,
    email: String,
    sign_in_cnt: u32,
    active: bool
}

// without copy trait without referencing an object,
// it will loose it's ownership as it's moved
#[derive(Debug)]
struct Rect{
    width: u32,
    height: u32
}

//Method
impl Rect{
    // same as function better to borrow with reference
    // otherwise will loose ownership
    fn area(&self)->u32 {
        self.width*self.height
    }

    fn can_hold(&self, other: &Rect)->bool {
        self.width>other.width && self.height>other.height
    }

    // associated functions which are used sometimes to create constructors 
    fn square(size: u32) -> Rect{
        Rect {
            width:size,
            height:size
        }
    }
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

    //area calculation

    let width: u32 = 5;
    let height: u32 = 9;
    let area = area_calc(width, height);

    println!("{}", area);

    //doing it with tuple

    let dimensions = (9, 5);
    let area_tuple = area_calc_tuple(dimensions);
    println!("{}", area_tuple);

    // We don't want to loose ownership
    let rectangle = Rect{
        height:5,
        width:10
    };

    println!("{:#?}", rectangle);

    println!("{}", rectangle.area());

    let area = area_calc_struct(&rectangle);

    println!("{} {}", area, rectangle.height);

    let a: u32 = 8;
    
    ret(a);

    println!("{}", a);



    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    let rect2 = Rect {
        width: 10,
        height: 40,
    };
    let rect3 = Rect {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rect::square(5);

    println!("{:#?}", sq);

}

fn ret(idx: u32) {
    let new_idx = idx;
    println!("{}", new_idx);
}

fn area_calc_struct(dim: &Rect)->u32{
    dim.height*dim.width
}

//tuple area calculation

fn area_calc_tuple(dim: (u32, u32)) -> u32{
    dim.0*dim.1
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


fn area_calc(width: u32, height: u32)->u32 {
    width*height
}
