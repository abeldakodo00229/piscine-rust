pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if horizontal("X", &table) || diagonals("X", &table) || vertical("X", &table) {
        return "player X won".to_string();
    } else if horizontal("O", &table) || diagonals("O", &table) || vertical("O", &table) {
        return "player O won".to_string();
    }
    "tie".to_string()
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    
    for elm in table{
            if (elm[0] == player && table[1][1] == player && table[2][2] == player) || (elm[2] == player && table[1][1] == player && table[2][0] == player){
                return true;
            }
    return false
    }
    return false
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut count : u32 = 0;
    for elm in table{
        for el in elm{
            if *el == player {
                count+=1;
            }
        }
        if count ==elm.len().try_into().unwrap(){
            return true;
        }
            count = 0;
        
    }
    return false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {

    for elm in table{
        for (i,el) in elm.iter().enumerate(){
            if *el == player && table[1][i] == player && table[2][i] == player{
                return true;
            }
        }
    return false
    }
    return false
}