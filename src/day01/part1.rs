
pub fn solution() {
    println!("Solution day one part one: {}", part_one());
    println!("Solution day one part two: {}", part_two());
}

fn part_one() -> u32 {
    let data = get_data();
    group_elf_inventories(data.as_slice())
        .iter()
        .map(|calories| calories.iter().sum()) // Sum each elf's inventory of calories
        .max()
        .unwrap()
}

fn part_two() -> u32 {
    let data = get_data();
    let mut elf_calories: Vec<u32> = group_elf_inventories(data.as_slice())
        .iter()
        .map(|calories| calories.iter().sum()) // Sum each elf's inventory of calories
        .collect();

    elf_calories.sort_by(|a, b| b.cmp(a));
    elf_calories.iter().take(3).sum()

}

fn get_data() -> Vec<Option<u32>> {
    let input = include_str!("input.txt");
    input.split('\n').map(|n| n.parse::<u32>().ok()).collect()
}

fn group_elf_inventories(calorie_list: &[Option<u32>]) -> Vec<Vec<u32>> {
    let mut grouped = Vec::new();
    let mut temp = Vec::new();
    for calories in calorie_list {
        match calories {
            None => {
                grouped.push(temp);
                temp = Vec::new();
            },
            Some(n) => temp.push(*n),
        }
    }
    grouped
}
