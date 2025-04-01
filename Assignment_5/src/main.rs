fn main(){
    let days = ["First",
                "second", 
                "third", 
                "fourth", 
                "fifth", 
                "sixth", 
                "seventh", 
                "eigth", 
                "ninth",
                "tenth",
                "eleventh",
                "twelfth"
                  ];
    
    let activities = ["A patridge in a pear tree",
                      "Two turtle doves",
                      "Three french hens",
                      "Four calling birds",
                      "Five gold rings",
                      "Six geese a-laying",
                      "Seven swans a-swimming",
                      "Eight maids a-milking",
                      "Nine ladies dancing",
                      "Ten lads a-leaping",
                      "Eleven piper piping",
                      "Twelve drummers drumming"
                      ];

                      for day in 0..12{
                        println!("On the {} day of christmas my true love gave to me\n", days[day]);

                        for activity in (0..=day).rev(){
                            if activity == 0 && day>0{
                                println!("And {}", activities[activity]);
                            }
                            else{
                                println!("{}", activities[activity]);
                            }
                            println!();
                        }
                      }




}
