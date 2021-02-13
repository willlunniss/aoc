use std::collections::HashMap;
use std::collections::HashSet;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
     // Process the questions, each group is seperate by a blank line
     let mut questions : HashSet<char> = HashSet::new();
     let mut question_count_sum = 0;
     for line in input.lines() {
         if line.is_empty() {
             // Blank line - end of group
             question_count_sum += questions.len();
             // Reset for next
             questions.clear();
             continue;
         }
         for q in line.chars() {
             // Increment the counter for the question
             questions.insert(q);
         }
     }
     if questions.len() > 0 {        
         // Process final group
         // Total questions answered yes to
         question_count_sum += questions.len();
     }
     return question_count_sum;
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
     // Process the questions, each group is seperate by a blank line
     let mut questions : HashMap<char, usize> = HashMap::new();
     let mut question_all_sum = 0;
     let mut group_size = 0;
     for line in input.lines() {
         if line.is_empty() {
             // Blank line - end of group
             // Count questions where everyone answered yes (responses == group size)
             question_all_sum += questions.values().filter(|&responses| *responses == group_size).count();
             // Reset for next
             questions.clear();
             group_size = 0;
             continue;
         }
          // Increment size of this group and then process this persons question
         group_size += 1;
         for q in line.chars() {
             // Increment the counter for the question
             questions.insert(q, 1 + if questions.contains_key(&q) { questions[&q] } else { 0 });
         }
     }
     if questions.keys().len() > 0 {        
         // Process final group
         // Questions where everyone answered yes
         question_all_sum += questions.values().filter(|&n| *n == group_size).count();
     }
     return question_all_sum;
}
