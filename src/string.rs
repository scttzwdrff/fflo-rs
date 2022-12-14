use crate::settings::*;
use crate::hand::*;
use crate::hands::*;
use crate::finger::*;

pub fn hand_FROM_string(string : &str) -> Hand {
    let mut fingers_as_strings  = string.split("\n").collect::<Vec<&str>>();
    let fingers : Vec<Finger> = fingers_as_strings.iter().map(|x| finger_FROM_string(x)).collect();
    Hand { fingers }
}
pub fn string_FROM_hand(hand : &Hand) -> String {
    let mut string = "".to_owned();
    for (i,finger) in hand.fingers.iter().enumerate() { 
        string.push_str(&string_FROM_finger(&finger));
        if i < hand.fingers.len() - 1 {
            string.push_str(&"\n".to_string());
        }
    }
    string
}
pub fn string_FROM_hands(hands : &Vec<Hand>) -> String {
    let mut string = "".to_owned();
    for (i,hand) in hands.iter().enumerate() { 
        string.push_str(&string_FROM_hand(&hand));
        if i < hands.len() - 1 {
            string.push_str(&"\n\n".to_string());
        }
    }
    string
}
pub fn hands_FROM_string(string : &str) -> Vec<Hand> {
    let mut hands_as_strings = string.split("\n\n").collect::<Vec<&str>>();
    hands_as_strings.iter().map(|x| hand_FROM_string(x)).collect()
}
pub fn string_FROM_finger(finger : &Finger) -> String {
    format!("{:+03},{:+03},{:+020.15}", finger.target.0, finger.target.1, finger.action)
}
pub fn finger_FROM_string(string : &str) -> Finger {
    let mut strings = string.split(",");
    let i = strings.next().unwrap().parse::<i32>().unwrap();
    let j = strings.next().unwrap().parse::<i32>().unwrap();
    let f = strings.next().unwrap().parse::<f64>().unwrap();
    Finger {
        target : (i,j),
        action : f,
    }
}
