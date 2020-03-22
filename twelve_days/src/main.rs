fn main() {
    let gifts = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Gold Rings",
        "Six Geese A-Laying",
        "Seven Swans A-Swimming",
        "Eight Maids A-Milking",
        "Nine Ladies Dancing",
        "Ten Lords A-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming"
    ];

    let mut num_end = "" ;
    for (day, gift) in gifts.iter().enumerate() {

        match day {
            0 => num_end = "st",
            1 => num_end = "nd",
            2 => num_end = "rd",
            _ => num_end = "th"
        }

        println!("On the {}{} day of Christmas my true love gave to me,", day+1, num_end);
        if day == 0 {
            println! ("{}.", gift);
            println! ("\n");
        } else {
            for x in (0..(day+1)).rev() {
                //to print this line at the end "if let"
                if let 0 = x {
                    println!("And {}.", gifts[0]);
                } 
                else {
                    println! ("{},", gifts[x]);
                
                }  
            }
            println! ("\n");
        }
    }
}