const NB_HIGHSCORES: usize = 5;

fn update_vec(v: &mut Vec<u32>, value: u32) -> bool {
    if v.len() < NB_HIGHSCORES {
        v.push(value);
        v.sort();
        true
    } else {
        for entry in v.iter_mut() {
            if value > *entry {
                *entry = value;
                return true;
            }
        }
        false
    }
}

fn print_game_information(tetris: &Tetris) {
    let mut new_highest_highscore = true;
    let mut new_highest_lines_sent = true;
    if let Some((mut highscores, mut lines_sent)) = 
     load_highscores_and_lines() {
        new_highest_highscore = update_vec(&mut highscores,  
         tetris.score);
        new_highest_lines_sent = update_vec(&mut lines_sent,  
         tetris.nb_lines);
        if new_highest_highscore || new_highest_lines_sent {
            save_highscores_and_lines(&highscores, &lines_sent);
        }
    } else {
        save_highscores_and_lines(&[tetris.score], &
         [tetris.nb_lines]);
    }
    println!("Game over...");
    println!("Score:           {}{}",
             tetris.score,
             if new_highest_highscore { " [NEW HIGHSCORE]"} else {   
              "" });
    println!("Number of lines: {}{}",
             tetris.nb_lines,
             if new_highest_lines_sent { " [NEW HIGHSCORE]"} else {  
              "" });
    println!("Current level:   {}", tetris.current_level);
}
