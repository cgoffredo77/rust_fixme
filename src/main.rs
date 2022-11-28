use std::fmt::{self, Formatter, Display};

/* Demonstrates printing of a user defined struct using println! macro.*/
///This program shows how to print a struct using println! without the debug trait.
///It first creates a struct named City giving it a name, latitude, and longitude.

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

///Then, it impliments the Display trait for the City struct, and helps format what the output of using println! on the struct would be.

impl Display for City {
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

///Next, we create a struct called color, giving it a u8 value for red, greed, and blue.

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

///The program then does the same thing for this struct as it did for City (implementing the Display trait and formatting).

impl Display for Color{
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {

   write!(f, "red: {},green: {},blue: {}", self.red, self.green, self.blue )
   }  
}

///In the main function of the code, we are simply making a City struct named 'city' and a Color struct named 'color'.
///We then give values to color and city, and then print these structs with println! (using {}, without the debug trait {:?})

fn main() {
    for city in [
        City { name: "Glassboro", lat: 39.702892, lon: -75.111839 },
        City { name: "Mullica Hill", lat: 39.73928, lon: -75.224072 },
        City { name: "Swedesboro", lat: 39.747616, lon: -75.310463 },
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Hint : Fix the code so you can print it using {}
        println!("{}", *color);
    }
}