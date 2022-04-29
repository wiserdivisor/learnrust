fn main(){
  // {} for taking value from args
  println!("{} days",31);

  // using positional args
  println!("{0} days in Jan and {1} days in Feb",31,28);

  // using named args
  println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumped over");

  // special formatting using ':'
  println!("Binary is as easy as {}-{:b}-{:b}",1,2,3);

  // 
}
