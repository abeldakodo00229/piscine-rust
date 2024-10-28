use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut result : HashMap<&str, usize> = HashMap::new();
    let mut n : usize =0;
    let mut bolean: bool =false;
    for word in &words{
     for (elm, _) in result.iter(){
        if word.to_string() == elm.to_string() {
            bolean=true;
        }
     }
     if !bolean{
         for word0 in &words{
            if word.to_string() ==word0.to_string() {
                n+=1;
            }
         }
         result.insert(word,n);
     }
     n=0;
     bolean=false;
    }
    return result;
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}